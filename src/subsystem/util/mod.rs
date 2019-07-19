pub mod areq;
pub mod sreq;
pub mod srsp;

use crate::error::Error;
use crate::frame::{MTExtendedHeader, MTHeader};
use crate::subsystem::MTFramePayload;
use crate::types::CommandType;
use std::io::Cursor;

pub fn try_decode(
    header: &MTHeader,
    _: &Option<MTExtendedHeader>,
    cursor: &mut Cursor<&[u8]>,
) -> Result<MTFramePayload, Error> {
    use MTFramePayload::*;

    match header.command.cmd_type {
        CommandType::POLL => Err(Error::NotImplemented),
        CommandType::SREQ => match header.command.id {
            0x06 => sreq::CallbackSubCmd::try_decode(cursor).map(|x| UTIL_CallbackSubCmd_SREQ(x)),
            0xee => sreq::GetExtAddr::try_decode(cursor).map(|x| UTIL_GetExtAddr_SREQ(x)),
            0x10 => sreq::Loopback::try_decode(cursor).map(|x| UTIL_Loopback_SREQ(x)),
            0x12 => sreq::Random::try_decode(cursor).map(|x| UTIL_Random_SREQ(x)),
            _ => Err(Error::NotImplemented),
        },
        CommandType::AREQ => match header.command.id {
            0x10 => areq::Loopback::try_decode(cursor).map(|x| UTIL_Loopback_AREQ(x)),
            _ => Err(Error::NotImplemented),
        },
        CommandType::SRSP => match header.command.id {
            0x06 => srsp::CallbackSubCmd::try_decode(cursor).map(|x| UTIL_CallbackSubCmd_SRSP(x)),
            0xee => srsp::GetExtAddr::try_decode(cursor).map(|x| UTIL_GetExtAddr_SRSP(x)),
            0x10 => srsp::Loopback::try_decode(cursor).map(|x| UTIL_Loopback_SRSP(x)),
            0x12 => srsp::Random::try_decode(cursor).map(|x| UTIL_Random_SRSP(x)),
            _ => Err(Error::NotImplemented),
        },
    }
}
