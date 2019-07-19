use crate::error::Error;
use crate::frame::{CommandCode, MTFrame, MTHeader};
use crate::subsystem::MTFramePayload;
use crate::types::*;
use bytes::{Buf, BufMut};
use std::io::Cursor;
use std::io::Read;

#[derive(Debug)]
pub struct Init {
    pub status: Status,
}

impl Init {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(Init { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x02,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_Init_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct DataReq {
    pub status: Status,
}

impl DataReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(DataReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x05,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_DataReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct PurgeReq {
    pub status: Status,
}

impl PurgeReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(PurgeReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x0e,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_PurgeReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct AssociateReq {
    pub status: Status,
}

impl AssociateReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(AssociateReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x06,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_AssociateReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct AssociateRsp {
    pub status: Status,
}

impl AssociateRsp {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(AssociateRsp { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x50,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_AssociateRsp_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct DisassociateReq {
    pub status: Status,
}

impl DisassociateReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(DisassociateReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x07,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_DisassociateReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct GetReq {
    pub status: Status,
    pub data: [u8; 16],
}

impl GetReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;

        let mut data: [u8; 16] = Default::default();
        cursor
            .read_exact(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;
        data.reverse();

        Ok(GetReq { status, data })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.status.encode_into(buffer);
        buffer.extend(self.data.iter().rev());
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x11,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SRSP,
                    subsystem: MTSubsystem::MAC,
                    id: 0x08,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_GetReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct SetReq {
    pub status: Status,
}

impl SetReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(SetReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x09,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_SetReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct SecurityGetReq {
    pub status: Status,
    pub index1: u8,
    pub index2: u8,
    pub data: Vec<u8>,
}

impl SecurityGetReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        let index1 = cursor.get_u8();
        let index2 = cursor.get_u8();

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(SecurityGetReq {
            status,
            index1,
            index2,
            data,
        })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.status.encode_into(buffer);
        buffer.put_u8(self.index1);
        buffer.put_u8(self.index2);
        buffer.extend(self.data.iter());
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x03 + self.data.len() as u8,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SRSP,
                    subsystem: MTSubsystem::MAC,
                    id: 0x30,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_SecurityGetReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct SecuritySetReq {
    pub status: Status,
}

impl SecuritySetReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(SecuritySetReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x31,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_SecuritySetReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct UpdatePANIdReq {
    pub status: Status,
}

impl UpdatePANIdReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(UpdatePANIdReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x32,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_UpdatePANIdReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct AddDeviceReq {
    pub status: Status,
}

impl AddDeviceReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(AddDeviceReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x33,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_AddDeviceReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct DeleteDeviceReq {
    pub status: Status,
}

impl DeleteDeviceReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(DeleteDeviceReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x34,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_DeleteDeviceReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct DeleteAllDevicesReq {
    pub status: Status,
}

impl DeleteAllDevicesReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(DeleteAllDevicesReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x35,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_DeleteAllDevicesReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct DeleteKeyReq {
    pub status: Status,
}

impl DeleteKeyReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(DeleteKeyReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x36,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_DeleteKeyReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct ReadKeyReq {
    pub status: Status,
    pub frame_counter: u32,
}

impl ReadKeyReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        let frame_counter = cursor.get_u32_le();
        Ok(ReadKeyReq {
            status,
            frame_counter,
        })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.status.encode_into(buffer);
        buffer.put_u32_le(self.frame_counter);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x05,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SRSP,
                    subsystem: MTSubsystem::MAC,
                    id: 0x37,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_ReadKeyReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct WriteKeyReq {
    pub status: Status,
}

impl WriteKeyReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(WriteKeyReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x38,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_WriteKeyReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct OrphanRsp {
    pub status: Status,
}

impl OrphanRsp {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(OrphanRsp { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x51,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_OrphanRsp_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct PollReq {
    pub status: Status,
}

impl PollReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(PollReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x0d,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_PollReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct ResetReq {
    pub status: Status,
}

impl ResetReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(ResetReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x01,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_ResetReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct ScanReq {
    pub status: Status,
}

impl ScanReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(ScanReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x0c,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_ScanReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct StartReq {
    pub status: Status,
}

impl StartReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(StartReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x03,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_StartReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct SyncReq {
    pub status: Status,
}

impl SyncReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(SyncReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x04,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_SyncReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct SetRxGainReq {
    pub status: Status,
}

impl SetRxGainReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(SetRxGainReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x0f,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_SetRxGainReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct WSAsyncReq {
    pub status: Status,
}

impl WSAsyncReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(WSAsyncReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x44,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_WSAsyncReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct FHEnableReq {
    pub status: Status,
}

impl FHEnableReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(FHEnableReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x40,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_FHEnableReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct FHStartReq {
    pub status: Status,
}

impl FHStartReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(FHStartReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x41,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_FHStartReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct FHGetReq {
    pub status: Status,
    pub data: Vec<u8>,
}

impl FHGetReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(FHGetReq { status, data })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.status.encode_into(buffer);
        buffer.extend(self.data.iter());
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x01 + self.data.len() as u8,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SRSP,
                    subsystem: MTSubsystem::MAC,
                    id: 0x42,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_FHGetReq_SRSP(self),
        }
    }
}

#[derive(Debug)]
pub struct FHSetReq {
    pub status: Status,
}

impl FHSetReq {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(FHSetReq { status })
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
                    subsystem: MTSubsystem::MAC,
                    id: 0x43,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_FHSetReq_SRSP(self),
        }
    }
}
