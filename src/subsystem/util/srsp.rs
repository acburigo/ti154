use crate::error::Error;
use crate::frame::{CommandCode, MTFrame, MTHeader};
use crate::subsystem::MTFramePayload;
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
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        let enables = cursor.get_u32_le();
        Ok(CallbackSubCmd { status, enables })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.status.encode_into(buffer);
        buffer.put_u32_le(self.enables);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x05,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SRSP,
                    subsystem: MTSubsystem::UTIL,
                    id: 0x06,
                },
            },
            extended_header: None,
            payload: MTFramePayload::UTIL_CallbackSubCmd_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct GetExtAddr {
    pub address_type: ExtendedAddressType,
    pub ext_address: ExtendedAddress,
}

impl GetExtAddr {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let address_type = ExtendedAddressType::try_decode(Read::by_ref(cursor))?;
        let ext_address = ExtendedAddress::try_decode(Read::by_ref(cursor))?;
        Ok(GetExtAddr {
            address_type,
            ext_address,
        })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.address_type.encode_into(buffer);
        self.ext_address.encode_into(buffer);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x09,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SRSP,
                    subsystem: MTSubsystem::UTIL,
                    id: 0xee,
                },
            },
            extended_header: None,
            payload: MTFramePayload::UTIL_GetExtAddr_SRSP(self),
        }
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

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x05 + self.data.len() as u8,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SRSP,
                    subsystem: MTSubsystem::UTIL,
                    id: 0x10,
                },
            },
            extended_header: None,
            payload: MTFramePayload::UTIL_Loopback_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct Random {
    pub number: u16,
}

impl Random {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let number = cursor.get_u16_le();
        Ok(Random { number })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u16_le(self.number);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x02,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SRSP,
                    subsystem: MTSubsystem::UTIL,
                    id: 0x12,
                },
            },
            extended_header: None,
            payload: MTFramePayload::UTIL_Random_SRSP(self),
        }
    }
}
