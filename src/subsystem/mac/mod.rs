pub mod areq;
pub mod sreq;
pub mod srsp;

use crate::error::Error;
use crate::frame::{MTHeader, MTExtendedHeader};
use crate::subsystem::MTFramePayload;
use crate::types::CommandType;
use std::io::Cursor;

pub fn try_from(
    header: &MTHeader,
    extended_header: &Option<MTExtendedHeader>,
    cursor: &mut Cursor<&[u8]>,
) -> Result<MTFramePayload, Error> {
    match header.command.cmd_type {
        CommandType::POLL => Err(Error::NotImplemented),
        CommandType::SREQ => Err(Error::NotImplemented),
        CommandType::AREQ => Err(Error::NotImplemented),
        CommandType::SRSP => Err(Error::NotImplemented),
    }
}
