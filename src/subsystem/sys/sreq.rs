use crate::error::Error;
use bytes::Buf;
use std::io::Cursor;
use std::io::Read;

#[derive(Debug)]
pub struct PingReq {}

impl PingReq {
    pub fn try_from(_: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        Ok(PingReq {})
    }
}

#[derive(Debug)]
pub struct VersionReq {}

impl VersionReq {
    pub fn try_from(_: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        Ok(VersionReq {})
    }
}

#[derive(Debug)]
pub struct NVCreateReq {
    pub sys_id: u8,
    pub item_id: u16,
    pub sub_id: u16,
    pub length: u32,
}

impl NVCreateReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let sys_id = cursor.get_u8();
        let item_id = cursor.get_u16_le();
        let sub_id = cursor.get_u16_le();
        let length = cursor.get_u32_le();
        Ok(NVCreateReq {
            sys_id,
            item_id,
            sub_id,
            length,
        })
    }
}

#[derive(Debug)]
pub struct NVDeleteReq {
    pub sys_id: u8,
    pub item_id: u16,
    pub sub_id: u16,
}

impl NVDeleteReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let sys_id = cursor.get_u8();
        let item_id = cursor.get_u16_le();
        let sub_id = cursor.get_u16_le();
        Ok(NVDeleteReq {
            sys_id,
            item_id,
            sub_id,
        })
    }
}

#[derive(Debug)]
pub struct NVLengthReq {
    pub sys_id: u8,
    pub item_id: u16,
    pub sub_id: u16,
}

impl NVLengthReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let sys_id = cursor.get_u8();
        let item_id = cursor.get_u16_le();
        let sub_id = cursor.get_u16_le();
        Ok(NVLengthReq {
            sys_id,
            item_id,
            sub_id,
        })
    }
}

#[derive(Debug)]
pub struct NVReadReq {
    pub sys_id: u8,
    pub item_id: u16,
    pub sub_id: u16,
    pub offset: u16,
    pub length: u8,
}

impl NVReadReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let sys_id = cursor.get_u8();
        let item_id = cursor.get_u16_le();
        let sub_id = cursor.get_u16_le();
        let offset = cursor.get_u16_le();
        let length = cursor.get_u8();
        Ok(NVReadReq {
            sys_id,
            item_id,
            sub_id,
            offset,
            length,
        })
    }
}

#[derive(Debug)]
pub struct NVWriteReq {
    pub sys_id: u8,
    pub item_id: u16,
    pub sub_id: u16,
    pub offset: u16,
    pub length: u8,
    pub data: Vec<u8>,
}

impl NVWriteReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let sys_id = cursor.get_u8();
        let item_id = cursor.get_u16_le();
        let sub_id = cursor.get_u16_le();
        let offset = cursor.get_u16_le();
        let length = cursor.get_u8();

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(NVWriteReq {
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
pub struct NVUpdateReq {
    pub sys_id: u8,
    pub item_id: u16,
    pub sub_id: u16,
    pub length: u8,
    pub data: Vec<u8>,
}

impl NVUpdateReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let sys_id = cursor.get_u8();
        let item_id = cursor.get_u16_le();
        let sub_id = cursor.get_u16_le();
        let length = cursor.get_u8();

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(NVUpdateReq {
            sys_id,
            item_id,
            sub_id,
            length,
            data,
        })
    }
}

#[derive(Debug)]
pub struct NVCompactReq {
    pub threshold: u16,
}

impl NVCompactReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let threshold = cursor.get_u16_le();
        Ok(NVCompactReq { threshold })
    }
}
