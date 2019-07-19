use crate::error::Error;
use crate::types::*;
use bytes::{Buf, BufMut};
use std::io::Cursor;
use std::io::Read;

#[derive(Debug)]
pub struct ResetReq {
    pub reset_type: ResetType,
}

impl ResetReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let reset_type = ResetType::try_from(Read::by_ref(cursor))?;
        Ok(ResetReq { reset_type })
    }

    pub fn try_into(&self, buffer: &mut Vec<u8>) {
        self.reset_type.try_into(buffer);
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

impl ResetInd {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
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

    pub fn try_into(&self, buffer: &mut Vec<u8>) {
        self.reason.try_into(buffer);
        self.transport.try_into(buffer);
        self.product.try_into(buffer);
        buffer.put_u8(self.major);
        buffer.put_u8(self.minor);
        buffer.put_u8(self.maint);
    }
}
