use crate::error::Error;
use crate::frame::{CommandCode, MTFrame, MTHeader};
use crate::subsystem::MTFramePayload;
use crate::types::*;
use bytes::{Buf, BufMut};
use std::io::Cursor;
use std::io::Read;

#[derive(Debug)]
pub struct PingReq {
    pub capabilities: u16,
}

impl PingReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let capabilities = cursor.get_u16_le();
        Ok(PingReq { capabilities })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u16_le(self.capabilities);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x01,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SRSP,
                    subsystem: MTSubsystem::SYS,
                    id: SYSCommandId::PingReq as u8,
                },
            },
            extended_header: None,
            payload: MTFramePayload::SYS_PingReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct VersionReq {
    pub transport: TransportProtocolRevision,
    pub product: ProductIdCode,
    pub major: u8,
    pub minor: u8,
    pub maint: u8,
}

impl VersionReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let transport = TransportProtocolRevision::try_decode(Read::by_ref(cursor))?;
        let product = ProductIdCode::try_decode(Read::by_ref(cursor))?;
        let major = cursor.get_u8();
        let minor = cursor.get_u8();
        let maint = cursor.get_u8();
        Ok(VersionReq {
            transport,
            product,
            major,
            minor,
            maint,
        })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.transport.encode_into(buffer);
        self.product.encode_into(buffer);
        buffer.put_u8(self.major);
        buffer.put_u8(self.minor);
        buffer.put_u8(self.maint);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x05,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SRSP,
                    subsystem: MTSubsystem::SYS,
                    id: SYSCommandId::VersionReq as u8,
                },
            },
            extended_header: None,
            payload: MTFramePayload::SYS_VersionReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct NVCreateReq {
    pub status: Status,
}

impl NVCreateReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(NVCreateReq { status })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.status.encode_into(buffer);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x01,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SRSP,
                    subsystem: MTSubsystem::SYS,
                    id: SYSCommandId::NVCreateReq as u8,
                },
            },
            extended_header: None,
            payload: MTFramePayload::SYS_NVCreateReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct NVDeleteReq {
    pub status: Status,
}

impl NVDeleteReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(NVDeleteReq { status })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.status.encode_into(buffer);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x01,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SRSP,
                    subsystem: MTSubsystem::SYS,
                    id: SYSCommandId::NVDeleteReq as u8,
                },
            },
            extended_header: None,
            payload: MTFramePayload::SYS_NVDeleteReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct NVLengthReq {
    pub length: u32,
}

impl NVLengthReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let length = cursor.get_u32_le();
        Ok(NVLengthReq { length })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u32_le(self.length);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x04,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SRSP,
                    subsystem: MTSubsystem::SYS,
                    id: SYSCommandId::NVLengthReq as u8,
                },
            },
            extended_header: None,
            payload: MTFramePayload::SYS_NVLengthReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct NVReadReq {
    pub status: Status,
    pub length: u8,
    pub data: Vec<u8>,
}

impl NVReadReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        let length = cursor.get_u8();

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(NVReadReq {
            status,
            length,
            data,
        })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.status.encode_into(buffer);
        buffer.put_u8(self.length);
        buffer.extend(self.data.iter());
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x02 + self.data.len() as u8,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SRSP,
                    subsystem: MTSubsystem::SYS,
                    id: SYSCommandId::NVReadReq as u8,
                },
            },
            extended_header: None,
            payload: MTFramePayload::SYS_NVReadReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct NVWriteReq {
    pub status: Status,
}

impl NVWriteReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(NVWriteReq { status })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.status.encode_into(buffer);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x01,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SRSP,
                    subsystem: MTSubsystem::SYS,
                    id: SYSCommandId::NVWriteReq as u8,
                },
            },
            extended_header: None,
            payload: MTFramePayload::SYS_NVWriteReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct NVUpdateReq {
    pub status: Status,
}

impl NVUpdateReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(NVUpdateReq { status })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.status.encode_into(buffer);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x01,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SRSP,
                    subsystem: MTSubsystem::SYS,
                    id: SYSCommandId::NVUpdateReq as u8,
                },
            },
            extended_header: None,
            payload: MTFramePayload::SYS_NVUpdateReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct NVCompactReq {
    pub status: Status,
}

impl NVCompactReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(NVCompactReq { status })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.status.encode_into(buffer);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x01,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SRSP,
                    subsystem: MTSubsystem::SYS,
                    id: SYSCommandId::NVCompactReq as u8,
                },
            },
            extended_header: None,
            payload: MTFramePayload::SYS_NVCompactReq_SRSP(self),
        }
    }
}
