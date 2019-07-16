use crate::error::Error;
use crate::types::*;
use bytes::Buf;
use std::convert::TryFrom;
use std::io::Cursor;
use std::io::Read;

#[derive(Debug)]
pub struct PingReq {
    pub capabilities: u16,
}

impl TryFrom<&mut Cursor<&[u8]>> for PingReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let capabilities = cursor.get_u16_le();
        Ok(PingReq { capabilities })
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

impl TryFrom<&mut Cursor<&[u8]>> for VersionReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
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
}

#[derive(Debug)]
pub struct NVCreateReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVCreateReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(NVCreateReq { status })
    }
}

#[derive(Debug)]
pub struct NVDeleteReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVDeleteReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(NVDeleteReq { status })
    }
}

#[derive(Debug)]
pub struct NVLengthReq {
    pub length: u32,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVLengthReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let length = cursor.get_u32_le();
        Ok(NVLengthReq { length })
    }
}

#[derive(Debug)]
pub struct NVReadReq {
    pub status: Status,
    pub length: u8,
    pub data: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVReadReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
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
}

#[derive(Debug)]
pub struct NVWriteReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVWriteReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(NVWriteReq { status })
    }
}

#[derive(Debug)]
pub struct NVUpdateReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVUpdateReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(NVUpdateReq { status })
    }
}

#[derive(Debug)]
pub struct NVCompactReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVCompactReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(NVCompactReq { status })
    }
}
