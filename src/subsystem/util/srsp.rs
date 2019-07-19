use crate::error::Error;
use crate::types::*;
use bytes::{Buf, BufMut};
use std::io::Cursor;
use std::io::Read;

#[derive(Debug)]
pub struct CallbackSubCmd {
    pub status: Status,
    pub enables: u32,
}

impl CallbackSubCmd {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let enables = cursor.get_u32_le();
        Ok(CallbackSubCmd { status, enables })
    }

    pub fn try_into(&self, buffer: &mut Vec<u8>) {
        self.status.try_into(buffer);
        buffer.put_u32_le(self.enables);
    }
}

#[derive(Debug)]
pub struct GetExtAddr {
    pub address_type: ExtendedAddressType,
    pub ext_address: ExtendedAddress,
}

impl GetExtAddr {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let address_type = ExtendedAddressType::try_from(Read::by_ref(cursor))?;
        let ext_address = ExtendedAddress::try_from(Read::by_ref(cursor))?;
        Ok(GetExtAddr {
            address_type,
            ext_address,
        })
    }

    pub fn try_into(&self, buffer: &mut Vec<u8>) {
        self.address_type.try_into(buffer);
        self.ext_address.try_into(buffer);
    }
}

#[derive(Debug)]
pub struct Loopback {
    pub repeats: u8,
    pub interval: u32,
    pub data: Vec<u8>,
}

impl Loopback {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
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

    pub fn try_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(self.repeats);
        buffer.put_u32_le(self.interval);
        buffer.extend(self.data.iter());
    }
}

#[derive(Debug)]
pub struct Random {
    pub number: u16,
}

impl Random {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let number = cursor.get_u16_le();
        Ok(Random { number })
    }

    pub fn try_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u16_le(self.number);
    }
}
