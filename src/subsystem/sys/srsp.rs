use crate::error::Error;
use crate::types::*;
use bytes::Buf;
use std::convert::TryFrom;
use std::io::Cursor;
use std::io::Read;

#[derive(Debug)]
pub struct PingReqSRSP {
    pub capabilities: u16,
}

impl TryFrom<&mut Cursor<&[u8]>> for PingReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let capabilities = cursor.get_u16_le();
        Ok(PingReqSRSP { capabilities })
    }
}

#[derive(Debug)]
pub struct VersionReqSRSP {
    pub transport: TransportProtocolRevision,
    pub product: ProductIdCode,
    pub major: u8,
    pub minor: u8,
    pub maint: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for VersionReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let transport = TransportProtocolRevision::try_from(Read::by_ref(cursor))?;
        let product = ProductIdCode::try_from(Read::by_ref(cursor))?;
        let major = cursor.get_u8();
        let minor = cursor.get_u8();
        let maint = cursor.get_u8();
        Ok(VersionReqSRSP {
            transport,
            product,
            major,
            minor,
            maint,
        })
    }
}

#[derive(Debug)]
pub struct NVCreateReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVCreateReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(NVCreateReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct NVDeleteReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVDeleteReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(NVDeleteReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct NVLengthReqSRSP {
    pub length: u32,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVLengthReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let length = cursor.get_u32_le();
        Ok(NVLengthReqSRSP { length })
    }
}

#[derive(Debug)]
pub struct NVReadReqSRSP {
    pub status: Status,
    pub length: u8,
    pub data: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVReadReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let length = cursor.get_u8();

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(NVReadReqSRSP {
            status,
            length,
            data,
        })
    }
}

#[derive(Debug)]
pub struct NVWriteReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVWriteReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(NVWriteReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct NVUpdateReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVUpdateReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(NVUpdateReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct NVCompactReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVCompactReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(NVCompactReqSRSP { status })
    }
}
