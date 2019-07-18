use crate::error::Error;
use crate::frame::{CommandCode, MTExtendedHeader, MTHeader};
use crate::subsystem::MTFramePayload;
use crate::types::{CommandType, ErrorCode};
use std::io::Cursor;

pub fn try_from(
    header: &MTHeader,
    _: &Option<MTExtendedHeader>,
    cursor: &mut Cursor<&[u8]>,
) -> Result<MTFramePayload, Error> {
    use MTFramePayload::*;

    match header.command.cmd_type {
        CommandType::POLL => Err(Error::NotImplemented),
        CommandType::SREQ => Err(Error::NotImplemented),
        CommandType::AREQ => Err(Error::NotImplemented),
        CommandType::SRSP => match header.command.id {
            0x00 => MTCommandError::try_from(cursor).map(|x| RPC_MTCommandError(x)),
            _ => Err(Error::NotImplemented),
        },
    }
}

#[derive(Debug)]
pub struct MTCommandError {
    pub error_code: ErrorCode,
    pub command: CommandCode,
}

impl MTCommandError {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let error_code = ErrorCode::try_from(cursor)?;
        let command = CommandCode::try_from(cursor)?;
        Ok(MTCommandError {
            error_code,
            command,
        })
    }

    pub fn try_into(&self, buffer: &mut Vec<u8>) {
        self.error_code.try_into(buffer);
        self.command.try_into(buffer);
    }
}
