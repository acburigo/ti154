pub mod areq;
pub mod sreq;
pub mod srsp;

use crate::error::Error;
use crate::frame::{MTExtendedHeader, MTHeader};
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
        CommandType::SREQ => match header.command.id {
            0x01 => Ok(SYS_PingReq_SREQ(sreq::PingReq::try_from(cursor)?)),
            0x02 => Ok(SYS_VersionReq_SREQ(sreq::VersionReq::try_from(cursor)?)),
            0x30 => Ok(SYS_NVCreateReq_SREQ(sreq::NVCreateReq::try_from(cursor)?)),
            0x31 => Ok(SYS_NVDeleteReq_SREQ(sreq::NVDeleteReq::try_from(cursor)?)),
            0x32 => Ok(SYS_NVLengthReq_SREQ(sreq::NVLengthReq::try_from(cursor)?)),
            0x33 => Ok(SYS_NVReadReq_SREQ(sreq::NVReadReq::try_from(cursor)?)),
            0x34 => Ok(SYS_NVWriteReq_SREQ(sreq::NVWriteReq::try_from(cursor)?)),
            0x35 => Ok(SYS_NVUpdateReq_SREQ(sreq::NVUpdateReq::try_from(cursor)?)),
            0x36 => Ok(SYS_NVCompactReq_SREQ(sreq::NVCompactReq::try_from(cursor)?)),
            _ => Err(Error::NotImplemented),
        },
        CommandType::AREQ => match header.command.id {
            0x00 => Ok(SYS_ResetReq_AREQ(areq::ResetReq::try_from(cursor)?)),
            0x80 => Ok(SYS_ResetInd_AREQ(areq::ResetInd::try_from(cursor)?)),
            _ => Err(Error::NotImplemented),
        },
        CommandType::SRSP => match header.command.id {
            0x01 => Ok(SYS_PingReq_SRSP(srsp::PingReq::try_from(cursor)?)),
            0x02 => Ok(SYS_VersionReq_SRSP(srsp::VersionReq::try_from(cursor)?)),
            0x30 => Ok(SYS_NVCreateReq_SRSP(srsp::NVCreateReq::try_from(cursor)?)),
            0x31 => Ok(SYS_NVDeleteReq_SRSP(srsp::NVDeleteReq::try_from(cursor)?)),
            0x32 => Ok(SYS_NVLengthReq_SRSP(srsp::NVLengthReq::try_from(cursor)?)),
            0x33 => Ok(SYS_NVReadReq_SRSP(srsp::NVReadReq::try_from(cursor)?)),
            0x34 => Ok(SYS_NVWriteReq_SRSP(srsp::NVWriteReq::try_from(cursor)?)),
            0x35 => Ok(SYS_NVUpdateReq_SRSP(srsp::NVUpdateReq::try_from(cursor)?)),
            0x36 => Ok(SYS_NVCompactReq_SRSP(srsp::NVCompactReq::try_from(cursor)?)),
            _ => Err(Error::NotImplemented),
        },
    }
}
