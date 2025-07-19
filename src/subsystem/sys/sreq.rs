use crate::error::Error;
use crate::frame::{CommandCode, MTFrame, MTHeader};
use crate::types::{CommandType, MTSubsystem, SYSCommandId};
use bytes::{Buf, BufMut};
use std::io::Cursor;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct PingReq {}

impl PingReq {
    pub fn try_decode(_: &[u8]) -> Result<Self, Error> {
        Ok(PingReq {})
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, _: &mut Vec<u8>) {}

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x00,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::SYS,
                    id: SYSCommandId::PingReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VersionReq {}

impl VersionReq {
    pub fn try_decode(_: &[u8]) -> Result<Self, Error> {
        Ok(VersionReq {})
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, _: &mut Vec<u8>) {}

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x00,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::SYS,
                    id: SYSCommandId::VersionReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NVCreateReq {
    pub sys_id: u8,
    pub item_id: u16,
    pub sub_id: u16,
    pub length: u32,
}

impl NVCreateReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
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

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(self.sys_id);
        buffer.put_u16_le(self.item_id);
        buffer.put_u16_le(self.sub_id);
        buffer.put_u32_le(self.length);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x09,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::SYS,
                    id: SYSCommandId::NVCreateReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NVDeleteReq {
    pub sys_id: u8,
    pub item_id: u16,
    pub sub_id: u16,
}

impl NVDeleteReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let sys_id = cursor.get_u8();
        let item_id = cursor.get_u16_le();
        let sub_id = cursor.get_u16_le();
        Ok(NVDeleteReq {
            sys_id,
            item_id,
            sub_id,
        })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(self.sys_id);
        buffer.put_u16_le(self.item_id);
        buffer.put_u16_le(self.sub_id);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x05,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::SYS,
                    id: SYSCommandId::NVDeleteReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NVLengthReq {
    pub sys_id: u8,
    pub item_id: u16,
    pub sub_id: u16,
}

impl NVLengthReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let sys_id = cursor.get_u8();
        let item_id = cursor.get_u16_le();
        let sub_id = cursor.get_u16_le();
        Ok(NVLengthReq {
            sys_id,
            item_id,
            sub_id,
        })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(self.sys_id);
        buffer.put_u16_le(self.item_id);
        buffer.put_u16_le(self.sub_id);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x05,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::SYS,
                    id: SYSCommandId::NVLengthReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NVReadReq {
    pub sys_id: u8,
    pub item_id: u16,
    pub sub_id: u16,
    pub offset: u16,
    pub length: u8,
}

impl NVReadReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
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

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(self.sys_id);
        buffer.put_u16_le(self.item_id);
        buffer.put_u16_le(self.sub_id);
        buffer.put_u16_le(self.offset);
        buffer.put_u8(self.length);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x08,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::SYS,
                    id: SYSCommandId::NVReadReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NVWriteReq {
    pub sys_id: u8,
    pub item_id: u16,
    pub sub_id: u16,
    pub offset: u16,
    pub length: u8,
    pub data: Vec<u8>,
}

impl NVWriteReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
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

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(self.sys_id);
        buffer.put_u16_le(self.item_id);
        buffer.put_u16_le(self.sub_id);
        buffer.put_u16_le(self.offset);
        buffer.put_u8(self.length);
        buffer.extend(self.data.iter());
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x08 + self.data.len() as u8,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::SYS,
                    id: SYSCommandId::NVWriteReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NVUpdateReq {
    pub sys_id: u8,
    pub item_id: u16,
    pub sub_id: u16,
    pub length: u8,
    pub data: Vec<u8>,
}

impl NVUpdateReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
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

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(self.sys_id);
        buffer.put_u16_le(self.item_id);
        buffer.put_u16_le(self.sub_id);
        buffer.put_u8(self.length);
        buffer.extend(self.data.iter());
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x06 + self.data.len() as u8,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::SYS,
                    id: SYSCommandId::NVUpdateReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NVCompactReq {
    pub threshold: u16,
}

impl NVCompactReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let threshold = cursor.get_u16_le();
        Ok(NVCompactReq { threshold })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u16_le(self.threshold);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x02,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::SYS,
                    id: SYSCommandId::NVCompactReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}
