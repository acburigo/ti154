use crate::error::Error;
use crate::frame::{CommandCode, MTExtendedHeader, MTHeader};
use crate::subsystem::MTFramePayload;
use crate::types::CommandType;
use bytes::Buf;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
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
            0x00 => Ok(RPC_MTCommandError(MTCommandError::try_from(cursor)?)),
            _ => Err(Error::NotImplemented),
        },
    }
}

#[derive(Debug, FromPrimitive)]
pub enum ErrorCode {
    InvalidSubsystem = 0x01,
    InvalidCommandId = 0x02,
    InvalidParameter = 0x03,
    InvalidLength = 0x04,
    UnsupportedExtendedHeaderType = 0x05,
    MemoryAllocationFailure = 0x06,
}

#[derive(Debug)]
pub struct MTCommandError {
    pub error_code: ErrorCode,
    pub command: CommandCode,
}

impl MTCommandError {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let error_code = cursor.get_u8();
        let error_code =
            FromPrimitive::from_u8(error_code).ok_or(Error::InvalidErrorCode(error_code))?;
        let command = CommandCode::try_from(cursor)?;

        Ok(MTCommandError {
            error_code,
            command,
        })
    }
}
