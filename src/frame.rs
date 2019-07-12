use crate::error::Error;
use crate::subsystem::MTSubsystem;
use bytes::Buf;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::convert::TryFrom;
use std::io::Cursor;

#[derive(Debug, FromPrimitive, PartialEq)]
pub enum CommandType {
    POLL = 0,
    SREQ = 1,
    AREQ = 2,
    SRSP = 3,
}

#[derive(Debug, FromPrimitive, PartialEq)]
pub enum MTExtendedHeaderStatus {
    Success = 0,                           // Success
    ResendLastFrame = 1,                   // Request - resend last frame
    UnsupportedStackId = 2,                // Unsupported Stack ID
    BlockOutOfOrder = 3,                   // Block out of order – fragmentation aborted
    BlockLengthChanged = 4,                // Block length changed – fragmentation aborted
    MemoryAllocationError = 5,             // Memory allocation error – fragmentation aborted
    FragmentationSequenceCompleted = 6,    // Fragmentation sequence completed
    FragmentationSequenceAborted = 7,      // Fragmentation sequence aborted
    UnsupportedFragmentationAckStatus = 8, // Unsupported Fragmentation Ack Status
}

#[derive(Debug)]
pub struct MTFrame {
    pub header: MTHeader,
    pub extended_header: Option<MTExtendedHeader>,
    pub payload: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for MTFrame {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let header = MTHeader::try_from(cursor.by_ref())?;
        let extended_header = if header.has_extension() {
            Some(MTExtendedHeader::try_from(cursor.by_ref())?)
        } else {
            None
        };

        Ok(MTFrame {
            header,
            extended_header,
            payload: Vec::new(),
        })
    }
}

#[derive(Debug)]
pub struct MTHeader {
    pub length: u8,
    pub command: CommandCode,
}

impl MTHeader {
    pub fn size() -> usize {
        return 3;
    }

    pub fn has_extension(&self) -> bool {
        self.command.is_extended
    }
}

impl TryFrom<&mut Cursor<&[u8]>> for MTHeader {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let length = cursor.get_u8();
        let command = CommandCode::try_from(cursor)?;
        Ok(MTHeader { length, command })
    }
}

#[derive(Debug)]
pub struct CommandCode {
    pub is_extended: bool,
    pub cmd_type: CommandType,
    pub subsystem: MTSubsystem,
    pub id: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for CommandCode {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let type_and_subsystem = cursor.get_u8();
        let id = cursor.get_u8();

        let is_extended = (type_and_subsystem & 0x80) != 0;

        let cmd_type = 0x03 & (type_and_subsystem >> 5);
        let cmd_type =
            FromPrimitive::from_u8(cmd_type).ok_or(Error::InvalidCommandType(cmd_type))?;

        let subsystem = type_and_subsystem & 0x1F;
        let subsystem =
            FromPrimitive::from_u8(subsystem).ok_or(Error::InvalidSubsystem(subsystem))?;

        Ok(CommandCode {
            is_extended,
            cmd_type,
            subsystem,
            id,
        })
    }
}

#[derive(Debug)]
pub enum MTExtendedHeader {
    V1 {
        stack_id: u8,
    },
    V2 {
        stack_id: u8,
        block: u8,
        packet_length: u16,
    },
    V3 {
        stack_id: u8,
        block: u8,
        status: MTExtendedHeaderStatus,
    },
    V4 {
        stack_id: u8,
        block: u8,
        status: MTExtendedHeaderStatus,
    },
}

impl TryFrom<&mut Cursor<&[u8]>> for MTExtendedHeader {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let version_and_stack_id = cursor.get_u8();
        let version = version_and_stack_id & 0xf8;
        let stack_id = version_and_stack_id & 0x07;

        if version == 1 {
            return Ok(MTExtendedHeader::V1 { stack_id });
        }

        let block = cursor.get_u8();

        if version == 2 {
            let packet_length = cursor.get_u16_le();
            return Ok(MTExtendedHeader::V2 {
                stack_id,
                block,
                packet_length,
            });
        }

        if version == 3 || version == 4 {
            let status = cursor.get_u8();
            let status =
                FromPrimitive::from_u8(status).ok_or(Error::InvalidExtendedHeaderStatus(status))?;

            if version == 3 {
                return Ok(MTExtendedHeader::V3 {
                    stack_id,
                    block,
                    status,
                });
            } else {
                return Ok(MTExtendedHeader::V4 {
                    stack_id,
                    block,
                    status,
                });
            }
        }

        Err(Error::NotImplemented)
    }
}
