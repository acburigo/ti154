use crate::error::Error;
use crate::types::*;
use bytes::Buf;
use std::convert::TryFrom;
use std::io::Cursor;
use std::io::Read;

#[derive(Debug)]
pub struct CallbackSubCmd {
    pub status: Status,
    pub enables: u32,
}

impl TryFrom<&mut Cursor<&[u8]>> for CallbackSubCmd {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let enables = cursor.get_u32_le();
        Ok(CallbackSubCmd { status, enables })
    }
}

#[derive(Debug)]
pub struct GetExtAddr {
    pub address_type: ExtendedAddressType,
    pub ext_address: ExtendedAddress,
}

impl TryFrom<&mut Cursor<&[u8]>> for GetExtAddr {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let address_type = ExtendedAddressType::try_from(Read::by_ref(cursor))?;
        let ext_address = ExtendedAddress::try_from(Read::by_ref(cursor))?;
        Ok(GetExtAddr {
            address_type,
            ext_address,
        })
    }
}

#[derive(Debug)]
pub struct Loopback {
    pub repeats: u8,
    pub interval: u32,
    pub data: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for Loopback {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let repeats = cursor.get_u8();
        let interval = cursor.get_u32_le();

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(Loopback {
            repeats,
            interval,
            data,
        })
    }
}

#[derive(Debug)]
pub struct Random {
    pub number: u16,
}

impl TryFrom<&mut Cursor<&[u8]>> for Random {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let number = cursor.get_u16_le();
        Ok(Random { number })
    }
}
