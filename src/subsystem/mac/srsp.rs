use crate::error::Error;
use crate::frame::{CommandCode, MTFrame, MTHeader};
use crate::types::*;
use bytes::{Buf, BufMut};
use std::io::Cursor;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct Init {
    pub status: Status,
}

impl Init {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(Init { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::Init as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataReq {
    pub status: Status,
}

impl DataReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(DataReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::DataReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PurgeReq {
    pub status: Status,
}

impl PurgeReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(PurgeReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::PurgeReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AssociateReq {
    pub status: Status,
}

impl AssociateReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(AssociateReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::AssociateReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AssociateRsp {
    pub status: Status,
}

impl AssociateRsp {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(AssociateRsp { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::AssociateRsp as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DisassociateReq {
    pub status: Status,
}

impl DisassociateReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(DisassociateReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::DisassociateReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetReq {
    pub status: Status,
    pub data: [u8; 16],
}

impl GetReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;

        let mut data: [u8; 16] = Default::default();
        cursor
            .read_exact(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;
        data.reverse();

        Ok(GetReq { status, data })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::GetReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SetReq {
    pub status: Status,
}

impl SetReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(SetReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::SetReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SecurityGetReq {
    pub status: Status,
    pub index1: u16,
    pub index2: u16,
    pub data: Vec<u8>,
}

impl SecurityGetReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        let index1 = cursor.get_u16_le();
        let index2 = cursor.get_u16_le();

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

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.status.encode_into(buffer);
        buffer.put_u16_le(self.index1);
        buffer.put_u16_le(self.index2);
        buffer.extend(self.data.iter());
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x05 + self.data.len() as u8,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SRSP,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::SecurityGetReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SecuritySetReq {
    pub status: Status,
}

impl SecuritySetReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(SecuritySetReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::SecuritySetReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UpdatePANIdReq {
    pub status: Status,
}

impl UpdatePANIdReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(UpdatePANIdReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::UpdatePANIdReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AddDeviceReq {
    pub status: Status,
}

impl AddDeviceReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(AddDeviceReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::AddDeviceReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeleteDeviceReq {
    pub status: Status,
}

impl DeleteDeviceReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(DeleteDeviceReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::DeleteDeviceReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeleteAllDevicesReq {
    pub status: Status,
}

impl DeleteAllDevicesReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(DeleteAllDevicesReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::DeleteAllDevicesReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeleteKeyReq {
    pub status: Status,
}

impl DeleteKeyReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(DeleteKeyReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::DeleteKeyReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReadKeyReq {
    pub status: Status,
    pub frame_counter: u32,
}

impl ReadKeyReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        let frame_counter = cursor.get_u32_le();
        Ok(ReadKeyReq {
            status,
            frame_counter,
        })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::ReadKeyReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WriteKeyReq {
    pub status: Status,
}

impl WriteKeyReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(WriteKeyReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::WriteKeyReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrphanRsp {
    pub status: Status,
}

impl OrphanRsp {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(OrphanRsp { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::OrphanRsp as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PollReq {
    pub status: Status,
}

impl PollReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(PollReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::PollReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResetReq {
    pub status: Status,
}

impl ResetReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(ResetReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::ResetReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScanReq {
    pub status: Status,
}

impl ScanReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(ScanReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::ScanReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StartReq {
    pub status: Status,
}

impl StartReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(StartReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::StartReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SyncReq {
    pub status: Status,
}

impl SyncReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(SyncReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::SyncReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SetRxGainReq {
    pub status: Status,
}

impl SetRxGainReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(SetRxGainReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::SetRxGainReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WSAsyncReq {
    pub status: Status,
}

impl WSAsyncReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(WSAsyncReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::WSAsyncReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FHEnableReq {
    pub status: Status,
}

impl FHEnableReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(FHEnableReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::FHEnableReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FHStartReq {
    pub status: Status,
}

impl FHStartReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(FHStartReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::FHStartReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FHGetReq {
    pub status: Status,
    pub data: Vec<u8>,
}

impl FHGetReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(FHGetReq { status, data })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::FHGetReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FHSetReq {
    pub status: Status,
}

impl FHSetReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let status = Status::try_decode(&mut cursor)?;
        Ok(FHSetReq { status })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
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
                    id: MACCommandId::FHSetReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}
