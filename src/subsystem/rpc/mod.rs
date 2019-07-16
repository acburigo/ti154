use crate::error::Error;
use crate::frame::CommandCode;
use bytes::Buf;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::convert::TryFrom;
use std::io::Cursor;

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

impl TryFrom<&mut Cursor<&[u8]>> for MTCommandError {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
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
