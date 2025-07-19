use crate::error::Error;
use crate::frame::{CommandCode, MTFrame, MTHeader};
use crate::subsystem::MTFramePayload;
use crate::types::{CommandType, ErrorCode, MTSubsystem, RPCCommandId};
use num_traits::FromPrimitive;
use std::io::Cursor;

pub fn try_decode(cmd_type: &CommandType, id: u8, buffer: &[u8]) -> Result<MTFramePayload, Error> {
    use MTFramePayload::*;

    let id = FromPrimitive::from_u8(id).ok_or(Error::InvalidCommandId(id))?;

    match cmd_type {
        CommandType::POLL => Err(Error::NotImplemented),
        CommandType::SREQ => Err(Error::NotImplemented),
        CommandType::AREQ => Err(Error::NotImplemented),
        CommandType::SRSP => match id {
            RPCCommandId::MTCommandError => {
                MTCommandError::try_decode(buffer).map(|x| RPC_MTCommandError(x))
            }
        },
    }
}

#[derive(Debug, Clone)]
pub struct MTCommandError {
    pub error_code: ErrorCode,
    pub command: CommandCode,
}

impl MTCommandError {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let error_code = ErrorCode::try_decode(&mut cursor)?;
        let command = CommandCode::try_decode(&mut cursor)?;
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
