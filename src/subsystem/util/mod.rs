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
    _: &Option<MTExtendedHeader>,
    cursor: &mut Cursor<&[u8]>,
) -> Result<MTFramePayload, Error> {
    use MTFramePayload::*;

    match header.command.cmd_type {
        CommandType::POLL => Err(Error::NotImplemented),
        CommandType::SREQ => {
            match header.command.id {
                0x06 => Ok(UTIL_CallbackSubCmd_SREQ(sreq::CallbackSubCmd::try_from(cursor)?)),
                0xee => Ok(UTIL_GetExtAddr_SREQ(sreq::GetExtAddr::try_from(cursor)?)),
                0x10 => Ok(UTIL_Loopback_SREQ(sreq::Loopback::try_from(cursor)?)),
                0x12 => Ok(UTIL_Random_SREQ(sreq::Random::try_from(cursor)?)),
                _ => Err(Error::NotImplemented),
            }
        }
        CommandType::AREQ => {
            match header.command.id {
                0x10 => Ok(UTIL_Loopback_AREQ(areq::Loopback::try_from(cursor)?)),
                _ => Err(Error::NotImplemented),
            }
        },
        CommandType::SRSP => {
            match header.command.id {
                0x06 => Ok(UTIL_CallbackSubCmd_SRSP(srsp::CallbackSubCmd::try_from(cursor)?)),
                0xee => Ok(UTIL_GetExtAddr_SRSP(srsp::GetExtAddr::try_from(cursor)?)),
                0x10 => Ok(UTIL_Loopback_SRSP(srsp::Loopback::try_from(cursor)?)),
                0x12 => Ok(UTIL_Random_SRSP(srsp::Random::try_from(cursor)?)),
                _ => Err(Error::NotImplemented),
            }
        }
    }
}
