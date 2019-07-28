pub mod areq;
pub mod sreq;
pub mod srsp;

use crate::error::Error;
use crate::subsystem::MTFramePayload;
use crate::types::CommandType;
use num_traits::FromPrimitive;

pub fn try_decode(cmd_type: &CommandType, id: u8, buffer: &[u8]) -> Result<MTFramePayload, Error> {
    use crate::types::SYSCommandId::*;
    use MTFramePayload::*;

    let id = FromPrimitive::from_u8(id).ok_or(Error::InvalidCommandId(id))?;

    match cmd_type {
        CommandType::POLL => Err(Error::NotImplemented),
        CommandType::SREQ => match id {
            PingReq => sreq::PingReq::try_decode(buffer).map(|x| SYS_PingReq_SREQ(x)),
            VersionReq => sreq::VersionReq::try_decode(buffer).map(|x| SYS_VersionReq_SREQ(x)),
            NVCreateReq => sreq::NVCreateReq::try_decode(buffer).map(|x| SYS_NVCreateReq_SREQ(x)),
            NVDeleteReq => sreq::NVDeleteReq::try_decode(buffer).map(|x| SYS_NVDeleteReq_SREQ(x)),
            NVLengthReq => sreq::NVLengthReq::try_decode(buffer).map(|x| SYS_NVLengthReq_SREQ(x)),
            NVReadReq => sreq::NVReadReq::try_decode(buffer).map(|x| SYS_NVReadReq_SREQ(x)),
            NVWriteReq => sreq::NVWriteReq::try_decode(buffer).map(|x| SYS_NVWriteReq_SREQ(x)),
            NVUpdateReq => sreq::NVUpdateReq::try_decode(buffer).map(|x| SYS_NVUpdateReq_SREQ(x)),
            NVCompactReq => {
                sreq::NVCompactReq::try_decode(buffer).map(|x| SYS_NVCompactReq_SREQ(x))
            }
            _ => Err(Error::NotImplemented),
        },
        CommandType::AREQ => match id {
            ResetReq => areq::ResetReq::try_decode(buffer).map(|x| SYS_ResetReq_AREQ(x)),
            ResetInd => areq::ResetInd::try_decode(buffer).map(|x| SYS_ResetInd_AREQ(x)),
            _ => Err(Error::NotImplemented),
        },
        CommandType::SRSP => match id {
            PingReq => srsp::PingReq::try_decode(buffer).map(|x| SYS_PingReq_SRSP(x)),
            VersionReq => srsp::VersionReq::try_decode(buffer).map(|x| SYS_VersionReq_SRSP(x)),
            NVCreateReq => srsp::NVCreateReq::try_decode(buffer).map(|x| SYS_NVCreateReq_SRSP(x)),
            NVDeleteReq => srsp::NVDeleteReq::try_decode(buffer).map(|x| SYS_NVDeleteReq_SRSP(x)),
            NVLengthReq => srsp::NVLengthReq::try_decode(buffer).map(|x| SYS_NVLengthReq_SRSP(x)),
            NVReadReq => srsp::NVReadReq::try_decode(buffer).map(|x| SYS_NVReadReq_SRSP(x)),
            NVWriteReq => srsp::NVWriteReq::try_decode(buffer).map(|x| SYS_NVWriteReq_SRSP(x)),
            NVUpdateReq => srsp::NVUpdateReq::try_decode(buffer).map(|x| SYS_NVUpdateReq_SRSP(x)),
            NVCompactReq => {
                srsp::NVCompactReq::try_decode(buffer).map(|x| SYS_NVCompactReq_SRSP(x))
            }
            _ => Err(Error::NotImplemented),
        },
    }
}
