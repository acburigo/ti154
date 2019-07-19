use crate::error::Error;
use crate::types::*;
use bytes::{Buf, BufMut};
use std::io::Cursor;
use std::io::Read;

#[derive(Debug)]
pub struct PingReq {
    pub capabilities: u16,
}

impl PingReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let capabilities = cursor.get_u16_le();
        Ok(PingReq { capabilities })
    }

    pub fn try_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u16_le(self.capabilities);
    }
}

#[derive(Debug)]
pub struct VersionReq {
    pub transport: TransportProtocolRevision,
    pub product: ProductIdCode,
    pub major: u8,
    pub minor: u8,
    pub maint: u8,
}

impl VersionReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let transport = TransportProtocolRevision::try_from(Read::by_ref(cursor))?;
        let product = ProductIdCode::try_from(Read::by_ref(cursor))?;
        let major = cursor.get_u8();
        let minor = cursor.get_u8();
        let maint = cursor.get_u8();
        Ok(VersionReq {
            transport,
            product,
            major,
            minor,
            maint,
        })
    }

    pub fn try_into(&self, buffer: &mut Vec<u8>) {
        self.transport.try_into(buffer);
        self.product.try_into(buffer);
        buffer.put_u8(self.major);
        buffer.put_u8(self.minor);
        buffer.put_u8(self.maint);
    }
}

#[derive(Debug)]
pub struct NVCreateReq {
    pub status: Status,
}

impl NVCreateReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(NVCreateReq { status })
    }

    pub fn try_into(&self, buffer: &mut Vec<u8>) {
        self.status.try_into(buffer);
    }
}

#[derive(Debug)]
pub struct NVDeleteReq {
    pub status: Status,
}

impl NVDeleteReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(NVDeleteReq { status })
    }

    pub fn try_into(&self, buffer: &mut Vec<u8>) {
        self.status.try_into(buffer);
    }
}

#[derive(Debug)]
pub struct NVLengthReq {
    pub length: u32,
}

impl NVLengthReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let length = cursor.get_u32_le();
        Ok(NVLengthReq { length })
    }

    pub fn try_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u32_le(self.length);
    }
}

#[derive(Debug)]
pub struct NVReadReq {
    pub status: Status,
    pub length: u8,
    pub data: Vec<u8>,
}

impl NVReadReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let length = cursor.get_u8();

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(NVReadReq {
            status,
            length,
            data,
        })
    }

    pub fn try_into(&self, buffer: &mut Vec<u8>) {
        self.status.try_into(buffer);
        buffer.put_u8(self.length);
        buffer.extend(self.data.iter());
    }
}

#[derive(Debug)]
pub struct NVWriteReq {
    pub status: Status,
}

impl NVWriteReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(NVWriteReq { status })
    }

    pub fn try_into(&self, buffer: &mut Vec<u8>) {
        self.status.try_into(buffer);
    }
}

#[derive(Debug)]
pub struct NVUpdateReq {
    pub status: Status,
}

impl NVUpdateReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(NVUpdateReq { status })
    }

    pub fn try_into(&self, buffer: &mut Vec<u8>) {
        self.status.try_into(buffer);
    }
}

#[derive(Debug)]
pub struct NVCompactReq {
    pub status: Status,
}

impl NVCompactReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(NVCompactReq { status })
    }

    pub fn try_into(&self, buffer: &mut Vec<u8>) {
        self.status.try_into(buffer);
    }
}
