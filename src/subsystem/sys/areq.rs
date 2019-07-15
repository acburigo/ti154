use crate::error::Error;
use crate::types::*;
use bytes::Buf;
use std::convert::TryFrom;
use std::io::Cursor;
use std::io::Read;

#[derive(Debug)]
pub struct ResetReq {
    pub reset_type: ResetType,
}

impl TryFrom<&mut Cursor<&[u8]>> for ResetReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let reset_type = ResetType::try_from(Read::by_ref(cursor))?;
        Ok(ResetReq { reset_type })
    }
}

#[derive(Debug)]
pub struct ResetInd {
    pub reason: ResetReason,
    pub transport: TransportProtocolRevision,
    pub product: ProductIdCode,
    pub major: u8,
    pub minor: u8,
    pub maint: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for ResetInd {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let reason = ResetReason::try_from(Read::by_ref(cursor))?;
        let transport = TransportProtocolRevision::try_from(Read::by_ref(cursor))?;
        let product = ProductIdCode::try_from(Read::by_ref(cursor))?;
        let major = cursor.get_u8();
        let minor = cursor.get_u8();
        let maint = cursor.get_u8();
        Ok(ResetInd {
            reason,
            transport,
            product,
            major,
            minor,
            maint,
        })
    }
}
