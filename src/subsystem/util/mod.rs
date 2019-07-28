pub mod areq;
pub mod sreq;
pub mod srsp;

use crate::error::Error;
use crate::subsystem::MTFramePayload;
use crate::types::CommandType;
use num_traits::FromPrimitive;

pub fn try_decode(cmd_type: &CommandType, id: u8, buffer: &[u8]) -> Result<MTFramePayload, Error> {
    use crate::types::UTILCommandId::*;
    use MTFramePayload::*;

    let id = FromPrimitive::from_u8(id).ok_or(Error::InvalidCommandId(id))?;

    match cmd_type {
        CommandType::POLL => Err(Error::NotImplemented),
        CommandType::SREQ => match id {
            CallbackSubCmd => {
                sreq::CallbackSubCmd::try_decode(buffer).map(|x| UTIL_CallbackSubCmd_SREQ(x))
            }
            GetExtAddr => sreq::GetExtAddr::try_decode(buffer).map(|x| UTIL_GetExtAddr_SREQ(x)),
            Loopback => sreq::Loopback::try_decode(buffer).map(|x| UTIL_Loopback_SREQ(x)),
            Random => sreq::Random::try_decode(buffer).map(|x| UTIL_Random_SREQ(x)),
        },
        CommandType::AREQ => match id {
            Loopback => areq::Loopback::try_decode(buffer).map(|x| UTIL_Loopback_AREQ(x)),
            _ => Err(Error::NotImplemented),
        },
        CommandType::SRSP => match id {
            CallbackSubCmd => {
                srsp::CallbackSubCmd::try_decode(buffer).map(|x| UTIL_CallbackSubCmd_SRSP(x))
            }
            GetExtAddr => srsp::GetExtAddr::try_decode(buffer).map(|x| UTIL_GetExtAddr_SRSP(x)),
            Loopback => srsp::Loopback::try_decode(buffer).map(|x| UTIL_Loopback_SRSP(x)),
            Random => srsp::Random::try_decode(buffer).map(|x| UTIL_Random_SRSP(x)),
        },
    }
}
