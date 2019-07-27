use crate::error::Error;
use crate::types::{CommandType, MTExtendedHeaderStatus, MTSubsystem};
use bytes::{Buf, BufMut};
use num_traits::FromPrimitive;
use std::io::{Cursor, Read};

#[derive(Debug, Clone)]
pub struct MTFrame {
    pub header: MTHeader,
    pub extended_header: Option<MTExtendedHeader>,
    pub payload: Vec<u8>,
}

impl MTFrame {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let header = MTHeader::try_decode(Read::by_ref(cursor))?;

        let extended_header = if header.has_extension() {
            Some(MTExtendedHeader::try_decode(Read::by_ref(cursor))?)
        } else {
            None
        };

        let mut payload = Vec::new();
        cursor
            .read_to_end(&mut payload)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(MTFrame {
            header,
            extended_header,
            payload,
        })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.header.encode_into(buffer);

        if let Some(ref extended_header) = self.extended_header {
            extended_header.encode_into(buffer);
        }

        buffer.extend(self.payload.iter());
    }

    pub fn encode_to_uart_transport_frame(&self) -> Vec<u8> {
        const START_OF_FRAME: u8 = 0xfe;
        let mut buffer = Vec::new();
        buffer.put_u8(START_OF_FRAME);
        self.encode_into(&mut buffer);
        let fcs = Self::compute_frame_check_sequence(&buffer[1..]);
        buffer.put_u8(fcs);
        buffer
    }

    pub fn compute_frame_check_sequence(mt_frame_bytes: &[u8]) -> u8 {
        mt_frame_bytes.iter().fold(0, |acc, x| acc ^ x)
    }
}

#[derive(Debug, Clone)]
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

    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let length = cursor.get_u8();
        let command = CommandCode::try_decode(cursor)?;
        Ok(MTHeader { length, command })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(self.length);
        self.command.encode_into(buffer);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CommandCode {
    pub is_extended: bool,
    pub cmd_type: CommandType,
    pub subsystem: MTSubsystem,
    pub id: u8,
}

impl CommandCode {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
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

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        let type_and_subsystem = {
            let value = ((self.cmd_type as u8) << 5) | (self.subsystem as u8);
            if self.is_extended {
                0x80 | value
            } else {
                value
            }
        };
        buffer.put_u8(type_and_subsystem);
        buffer.put_u8(self.id);
    }
}

#[derive(Debug, Clone)]
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

impl MTExtendedHeader {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let version_and_stack_id = cursor.get_u8();
        let version = (version_and_stack_id & 0xf8) >> 3;
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
            let status = MTExtendedHeaderStatus::try_decode(cursor)?;

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

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        match self {
            MTExtendedHeader::V1 { stack_id } => {
                let version_and_stack_id = (1 << 3) | stack_id;
                buffer.put_u8(version_and_stack_id);
            }
            MTExtendedHeader::V2 {
                stack_id,
                block,
                packet_length,
            } => {
                let version_and_stack_id = (2 << 3) | stack_id;
                buffer.put_u8(version_and_stack_id);
                buffer.put_u8(*block);
                buffer.put_u16_le(*packet_length);
            }
            MTExtendedHeader::V3 {
                stack_id,
                block,
                status,
            } => {
                let version_and_stack_id = (3 << 3) | stack_id;
                buffer.put_u8(version_and_stack_id);
                buffer.put_u8(*block);
                status.encode_into(buffer);
            }
            MTExtendedHeader::V4 {
                stack_id,
                block,
                status,
            } => {
                let version_and_stack_id = (4 << 3) | stack_id;
                buffer.put_u8(version_and_stack_id);
                buffer.put_u8(*block);
                status.encode_into(buffer);
            }
        }
    }
}
