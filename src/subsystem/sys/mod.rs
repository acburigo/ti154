use crate::error::Error;
use crate::types::*;
use bytes::Buf;
use std::convert::TryFrom;
use std::io::Cursor;
use std::io::Read;

#[derive(Debug)]
pub struct ResetReqAREQ {
    pub reset_type: ResetType,
}

impl TryFrom<&mut Cursor<&[u8]>> for ResetReqAREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let reset_type = ResetType::try_from(Read::by_ref(cursor))?;
        Ok(ResetReqAREQ { reset_type })
    }
}

#[derive(Debug)]
pub struct PingReqSREQ {}

impl TryFrom<&mut Cursor<&[u8]>> for PingReqSREQ {
    type Error = Error;
    fn try_from(_: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        Ok(PingReqSREQ {})
    }
}

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
pub struct VersionReqSREQ {}

impl TryFrom<&mut Cursor<&[u8]>> for VersionReqSREQ {
    type Error = Error;
    fn try_from(_: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        Ok(VersionReqSREQ {})
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
pub struct NVCreateReqSREQ {
    pub sys_id: u8,
    pub item_id: u16,
    pub sub_id: u16,
    pub length: u32,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVCreateReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let sys_id = cursor.get_u8();
        let item_id = cursor.get_u16_le();
        let sub_id = cursor.get_u16_le();
        let length = cursor.get_u32_le();
        Ok(NVCreateReqSREQ {
            sys_id,
            item_id,
            sub_id,
            length,
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
pub struct NVDeleteReqSREQ {
    pub sys_id: u8,
    pub item_id: u16,
    pub sub_id: u16,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVDeleteReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let sys_id = cursor.get_u8();
        let item_id = cursor.get_u16_le();
        let sub_id = cursor.get_u16_le();
        Ok(NVDeleteReqSREQ {
            sys_id,
            item_id,
            sub_id,
        })
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
pub struct NVLengthReqSREQ {
    pub sys_id: u8,
    pub item_id: u16,
    pub sub_id: u16,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVLengthReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let sys_id = cursor.get_u8();
        let item_id = cursor.get_u16_le();
        let sub_id = cursor.get_u16_le();
        Ok(NVLengthReqSREQ {
            sys_id,
            item_id,
            sub_id,
        })
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
pub struct NVReadReqSREQ {
    pub sys_id: u8,
    pub item_id: u16,
    pub sub_id: u16,
    pub offset: u16,
    pub length: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVReadReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let sys_id = cursor.get_u8();
        let item_id = cursor.get_u16_le();
        let sub_id = cursor.get_u16_le();
        let offset = cursor.get_u16_le();
        let length = cursor.get_u8();
        Ok(NVReadReqSREQ {
            sys_id,
            item_id,
            sub_id,
            offset,
            length,
        })
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
pub struct NVWriteReqSREQ {
    pub sys_id: u8,
    pub item_id: u16,
    pub sub_id: u16,
    pub offset: u16,
    pub length: u8,
    pub data: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVWriteReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let sys_id = cursor.get_u8();
        let item_id = cursor.get_u16_le();
        let sub_id = cursor.get_u16_le();
        let offset = cursor.get_u16_le();
        let length = cursor.get_u8();

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(NVWriteReqSREQ {
            sys_id,
            item_id,
            sub_id,
            offset,
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
pub struct NVUpdateReqSREQ {
    pub sys_id: u8,
    pub item_id: u16,
    pub sub_id: u16,
    pub length: u8,
    pub data: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVUpdateReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let sys_id = cursor.get_u8();
        let item_id = cursor.get_u16_le();
        let sub_id = cursor.get_u16_le();
        let length = cursor.get_u8();

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(NVUpdateReqSREQ {
            sys_id,
            item_id,
            sub_id,
            length,
            data,
        })
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
pub struct NVCompactReqSREQ {
    pub threshold: u16,
}

impl TryFrom<&mut Cursor<&[u8]>> for NVCompactReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let threshold = cursor.get_u16_le();
        Ok(NVCompactReqSREQ { threshold })
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

#[derive(Debug)]
pub struct ResetIndAREQ {
    pub reason: ResetReason,
    pub transport: TransportProtocolRevision,
    pub product: ProductIdCode,
    pub major: u8,
    pub minor: u8,
    pub maint: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for ResetIndAREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let reason = ResetReason::try_from(Read::by_ref(cursor))?;
        let transport = TransportProtocolRevision::try_from(Read::by_ref(cursor))?;
        let product = ProductIdCode::try_from(Read::by_ref(cursor))?;
        let major = cursor.get_u8();
        let minor = cursor.get_u8();
        let maint = cursor.get_u8();
        Ok(ResetIndAREQ {
            reason,
            transport,
            product,
            major,
            minor,
            maint,
        })
    }
}
