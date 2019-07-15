use crate::error::Error;
use crate::types::*;
use bytes::Buf;
use std::convert::TryFrom;
use std::io::Cursor;
use std::io::Read;

#[derive(Debug)]
pub struct Init {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for Init {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(Init { status })
    }
}

#[derive(Debug)]
pub struct DataReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for DataReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(DataReq { status })
    }
}

#[derive(Debug)]
pub struct PurgeReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for PurgeReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(PurgeReq { status })
    }
}

#[derive(Debug)]
pub struct AssociateReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for AssociateReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(AssociateReq { status })
    }
}

#[derive(Debug)]
pub struct AssociateRsp {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for AssociateRsp {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(AssociateRsp { status })
    }
}

#[derive(Debug)]
pub struct DisassociateReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for DisassociateReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(DisassociateReq { status })
    }
}

#[derive(Debug)]
pub struct GetReq {
    pub status: Status,
    pub data: [u8; 16],
}

impl TryFrom<&mut Cursor<&[u8]>> for GetReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;

        let mut data: [u8; 16] = Default::default();
        cursor
            .read_exact(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;
        data.reverse();

        Ok(GetReq { status, data })
    }
}

#[derive(Debug)]
pub struct SetReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for SetReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(SetReq { status })
    }
}

#[derive(Debug)]
pub struct SecurityGetReq {
    pub status: Status,
    pub index1: u8,
    pub index2: u8,
    pub data: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for SecurityGetReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
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
}

#[derive(Debug)]
pub struct SecuritySetReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for SecuritySetReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(SecuritySetReq { status })
    }
}

#[derive(Debug)]
pub struct UpdatePANIdReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for UpdatePANIdReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(UpdatePANIdReq { status })
    }
}

#[derive(Debug)]
pub struct AddDeviceReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for AddDeviceReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(AddDeviceReq { status })
    }
}

#[derive(Debug)]
pub struct DeleteDeviceReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for DeleteDeviceReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(DeleteDeviceReq { status })
    }
}

#[derive(Debug)]
pub struct DeleteAllDevicesReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for DeleteAllDevicesReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(DeleteAllDevicesReq { status })
    }
}

#[derive(Debug)]
pub struct DeleteKeyReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for DeleteKeyReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(DeleteKeyReq { status })
    }
}

#[derive(Debug)]
pub struct ReadKeyReq {
    pub status: Status,
    pub frame_counter: u32,
}

impl TryFrom<&mut Cursor<&[u8]>> for ReadKeyReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let frame_counter = cursor.get_u32_le();
        Ok(ReadKeyReq {
            status,
            frame_counter,
        })
    }
}

#[derive(Debug)]
pub struct WriteKeyReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for WriteKeyReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(WriteKeyReq { status })
    }
}

#[derive(Debug)]
pub struct OrphanRsp {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for OrphanRsp {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(OrphanRsp { status })
    }
}

#[derive(Debug)]
pub struct PollReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for PollReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(PollReq { status })
    }
}

#[derive(Debug)]
pub struct ResetReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for ResetReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(ResetReq { status })
    }
}

#[derive(Debug)]
pub struct ScanReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for ScanReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(ScanReq { status })
    }
}

#[derive(Debug)]
pub struct StartReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for StartReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(StartReq { status })
    }
}

#[derive(Debug)]
pub struct SyncReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for SyncReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(SyncReq { status })
    }
}

#[derive(Debug)]
pub struct SetRxGainReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for SetRxGainReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(SetRxGainReq { status })
    }
}

#[derive(Debug)]
pub struct WSAsyncReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for WSAsyncReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(WSAsyncReq { status })
    }
}

#[derive(Debug)]
pub struct FHEnableReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for FHEnableReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(FHEnableReq { status })
    }
}

#[derive(Debug)]
pub struct FHStartReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for FHStartReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(FHStartReq { status })
    }
}

#[derive(Debug)]
pub struct FHGetReq {
    pub status: Status,
    pub data: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for FHGetReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(FHGetReq { status, data })
    }
}

#[derive(Debug)]
pub struct FHSetReq {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for FHSetReq {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(FHSetReq { status })
    }
}
