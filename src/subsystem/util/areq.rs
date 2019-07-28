use crate::error::Error;
use crate::frame::{CommandCode, MTFrame, MTHeader};
use crate::types::{CommandType, MTSubsystem, UTILCommandId};
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
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
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

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(self.repeats);
        buffer.put_u32_le(self.interval);
        buffer.extend(self.data.iter());
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x05 + self.data.len() as u8,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::AREQ,
                    subsystem: MTSubsystem::UTIL,
                    id: UTILCommandId::Loopback as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}
