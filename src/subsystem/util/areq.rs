use crate::error::Error;
use bytes::{Buf, BufMut};
use std::io::Cursor;
use std::io::Read;

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
