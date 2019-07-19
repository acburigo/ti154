use crate::error::Error;
use crate::types::*;
use bytes::{Buf, BufMut};
use std::io::Cursor;
use std::io::Read;

#[derive(Debug)]
pub struct CallbackSubCmd {
    pub subsystem_id: SubsystemId,
    pub enables: u32,
}

impl CallbackSubCmd {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let subsystem_id = SubsystemId::try_decode(Read::by_ref(cursor))?;
        let enables = cursor.get_u32_le();
        Ok(CallbackSubCmd {
            subsystem_id,
            enables,
        })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.subsystem_id.encode_into(buffer);
        buffer.put_u32_le(self.enables);
    }
}

#[derive(Debug)]
pub struct GetExtAddr {
    pub address_type: ExtendedAddressType,
}

impl GetExtAddr {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let address_type = ExtendedAddressType::try_decode(Read::by_ref(cursor))?;
        Ok(GetExtAddr { address_type })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.address_type.encode_into(buffer);
    }
}

#[derive(Debug)]
pub struct Loopback {
    pub repeats: u8,
    pub interval: u32,
    pub data: Vec<u8>,
}

impl Loopback {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
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

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(self.repeats);
        buffer.put_u32_le(self.interval);
        buffer.extend(self.data.iter());
    }
}

#[derive(Debug)]
pub struct Random {}

impl Random {
    pub fn try_decode(_: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        Ok(Random {})
    }

    pub fn encode_into(&self, _: &mut Vec<u8>) {}
}
