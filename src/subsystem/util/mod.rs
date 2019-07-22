pub mod areq;
pub mod sreq;
pub mod srsp;

use crate::error::Error;
use crate::frame::{MTExtendedHeader, MTHeader};
use crate::subsystem::MTFramePayload;
use crate::types::CommandType;
use num_traits::FromPrimitive;
use std::io::Cursor;

pub fn try_decode(
    header: &MTHeader,
    _: &Option<MTExtendedHeader>,
    cursor: &mut Cursor<&[u8]>,
) -> Result<MTFramePayload, Error> {
    use crate::types::UTILCommandId::*;
    use MTFramePayload::*;

    let command_id =
        FromPrimitive::from_u8(header.command.id).ok_or(Error::InvalidStatus(header.command.id))?;

    match header.command.cmd_type {
        CommandType::POLL => Err(Error::NotImplemented),
        CommandType::SREQ => match command_id {
            CallbackSubCmd => {
                sreq::CallbackSubCmd::try_decode(cursor).map(|x| UTIL_CallbackSubCmd_SREQ(x))
            }
            GetExtAddr => sreq::GetExtAddr::try_decode(cursor).map(|x| UTIL_GetExtAddr_SREQ(x)),
            Loopback => sreq::Loopback::try_decode(cursor).map(|x| UTIL_Loopback_SREQ(x)),
            Random => sreq::Random::try_decode(cursor).map(|x| UTIL_Random_SREQ(x)),
        },
        CommandType::AREQ => match command_id {
            Loopback => areq::Loopback::try_decode(cursor).map(|x| UTIL_Loopback_AREQ(x)),
            _ => Err(Error::NotImplemented),
        },
        CommandType::SRSP => match command_id {
            CallbackSubCmd => {
                srsp::CallbackSubCmd::try_decode(cursor).map(|x| UTIL_CallbackSubCmd_SRSP(x))
            }
            GetExtAddr => srsp::GetExtAddr::try_decode(cursor).map(|x| UTIL_GetExtAddr_SRSP(x)),
            Loopback => srsp::Loopback::try_decode(cursor).map(|x| UTIL_Loopback_SRSP(x)),
            Random => srsp::Random::try_decode(cursor).map(|x| UTIL_Random_SRSP(x)),
        },
    }
}
