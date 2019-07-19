use crate::error::Error;
use crate::frame::{CommandCode, MTFrame, MTHeader};
use crate::subsystem::MTFramePayload;
use crate::types::*;
use bytes::{Buf, BufMut};
use std::io::Cursor;
use std::io::Read;

#[derive(Debug)]
pub struct ResetReq {
    pub reset_type: ResetType,
}

impl ResetReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let reset_type = ResetType::try_decode(Read::by_ref(cursor))?;
        Ok(ResetReq { reset_type })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.reset_type.encode_into(buffer);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x01,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::AREQ,
                    subsystem: MTSubsystem::SYS,
                    id: 0x00,
                },
            },
            extended_header: None,
            payload: MTFramePayload::SYS_ResetReq_AREQ(self),
        }
    }
}

#[derive(Debug)]
pub struct ResetInd {
    pub reason: ResetReason,
    pub transport: TransportProtocolRevision,
    pub product: ProductIdCode,
    pub major: u8,
    pub minor: u8,
    pub maint: u8,
}

impl ResetInd {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let reason = ResetReason::try_decode(Read::by_ref(cursor))?;
        let transport = TransportProtocolRevision::try_decode(Read::by_ref(cursor))?;
        let product = ProductIdCode::try_decode(Read::by_ref(cursor))?;
        let major = cursor.get_u8();
        let minor = cursor.get_u8();
        let maint = cursor.get_u8();
        Ok(ResetInd {
            reason,
            transport,
            product,
            major,
            minor,
            maint,
        })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.reason.encode_into(buffer);
        self.transport.encode_into(buffer);
        self.product.encode_into(buffer);
        buffer.put_u8(self.major);
        buffer.put_u8(self.minor);
        buffer.put_u8(self.maint);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x06,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::AREQ,
                    subsystem: MTSubsystem::SYS,
                    id: 0x80,
                },
            },
            extended_header: None,
            payload: MTFramePayload::SYS_ResetInd_AREQ(self),
        }
    }
}
