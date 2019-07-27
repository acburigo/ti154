use crate::error::Error;
use crate::frame::{CommandCode, MTFrame, MTHeader};
use crate::subsystem::MTFramePayload;
use crate::types::{CommandType, ErrorCode, MTSubsystem, RPCCommandId};
use num_traits::FromPrimitive;
use std::io::Cursor;

pub fn try_decode(
    header: &MTHeader,
    cursor: &mut Cursor<&[u8]>,
) -> Result<MTFramePayload, Error> {
    use MTFramePayload::*;

    let command_id =
        FromPrimitive::from_u8(header.command.id).ok_or(Error::InvalidStatus(header.command.id))?;

    match header.command.cmd_type {
        CommandType::POLL => Err(Error::NotImplemented),
        CommandType::SREQ => Err(Error::NotImplemented),
        CommandType::AREQ => Err(Error::NotImplemented),
        CommandType::SRSP => match command_id {
            RPCCommandId::MTCommandError => {
                MTCommandError::try_decode(cursor).map(|x| RPC_MTCommandError(x))
            }
        },
    }
}

#[derive(Debug)]
pub struct MTCommandError {
    pub error_code: ErrorCode,
    pub command: CommandCode,
}

impl MTCommandError {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let error_code = ErrorCode::try_decode(cursor)?;
        let command = CommandCode::try_decode(cursor)?;
        Ok(MTCommandError {
            error_code,
            command,
        })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.error_code.encode_into(buffer);
        self.command.encode_into(buffer);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x03,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SRSP,
                    subsystem: MTSubsystem::RPC,
                    id: RPCCommandId::MTCommandError as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}
