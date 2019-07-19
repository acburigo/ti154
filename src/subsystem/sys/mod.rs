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
            0x01 => sreq::PingReq::try_decode(cursor).map(|x| SYS_PingReq_SREQ(x)),
            0x02 => sreq::VersionReq::try_decode(cursor).map(|x| SYS_VersionReq_SREQ(x)),
            0x30 => sreq::NVCreateReq::try_decode(cursor).map(|x| SYS_NVCreateReq_SREQ(x)),
            0x31 => sreq::NVDeleteReq::try_decode(cursor).map(|x| SYS_NVDeleteReq_SREQ(x)),
            0x32 => sreq::NVLengthReq::try_decode(cursor).map(|x| SYS_NVLengthReq_SREQ(x)),
            0x33 => sreq::NVReadReq::try_decode(cursor).map(|x| SYS_NVReadReq_SREQ(x)),
            0x34 => sreq::NVWriteReq::try_decode(cursor).map(|x| SYS_NVWriteReq_SREQ(x)),
            0x35 => sreq::NVUpdateReq::try_decode(cursor).map(|x| SYS_NVUpdateReq_SREQ(x)),
            0x36 => sreq::NVCompactReq::try_decode(cursor).map(|x| SYS_NVCompactReq_SREQ(x)),
            _ => Err(Error::NotImplemented),
        },
        CommandType::AREQ => match header.command.id {
            0x00 => areq::ResetReq::try_decode(cursor).map(|x| SYS_ResetReq_AREQ(x)),
            0x80 => areq::ResetInd::try_decode(cursor).map(|x| SYS_ResetInd_AREQ(x)),
            _ => Err(Error::NotImplemented),
        },
        CommandType::SRSP => match header.command.id {
            0x01 => srsp::PingReq::try_decode(cursor).map(|x| SYS_PingReq_SRSP(x)),
            0x02 => srsp::VersionReq::try_decode(cursor).map(|x| SYS_VersionReq_SRSP(x)),
            0x30 => srsp::NVCreateReq::try_decode(cursor).map(|x| SYS_NVCreateReq_SRSP(x)),
            0x31 => srsp::NVDeleteReq::try_decode(cursor).map(|x| SYS_NVDeleteReq_SRSP(x)),
            0x32 => srsp::NVLengthReq::try_decode(cursor).map(|x| SYS_NVLengthReq_SRSP(x)),
            0x33 => srsp::NVReadReq::try_decode(cursor).map(|x| SYS_NVReadReq_SRSP(x)),
            0x34 => srsp::NVWriteReq::try_decode(cursor).map(|x| SYS_NVWriteReq_SRSP(x)),
            0x35 => srsp::NVUpdateReq::try_decode(cursor).map(|x| SYS_NVUpdateReq_SRSP(x)),
            0x36 => srsp::NVCompactReq::try_decode(cursor).map(|x| SYS_NVCompactReq_SRSP(x)),
            _ => Err(Error::NotImplemented),
        },
    }
}
