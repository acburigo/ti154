use crate::error::Error;
use crate::types::*;
use bytes::Buf;
use std::convert::TryFrom;
use std::io::Cursor;
use std::io::Read;

#[derive(Debug)]
pub struct CallbackSubCmdSREQ {
    pub subsystem_id: SubsystemId,
    pub enables: u32,
}

impl TryFrom<&mut Cursor<&[u8]>> for CallbackSubCmdSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let subsystem_id = SubsystemId::try_from(Read::by_ref(cursor))?;
        let enables = cursor.get_u32_le();
        Ok(CallbackSubCmdSREQ {
            subsystem_id,
            enables,
        })
    }
}

#[derive(Debug)]
pub struct CallbackSubCmdSRSP {
    pub status: Status,
    pub enables: u32,
}

impl TryFrom<&mut Cursor<&[u8]>> for CallbackSubCmdSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let enables = cursor.get_u32_le();
        Ok(CallbackSubCmdSRSP { status, enables })
    }
}

#[derive(Debug)]
pub struct GetExtAddrSREQ {
    pub address_type: ExtendedAddressType,
}

impl TryFrom<&mut Cursor<&[u8]>> for GetExtAddrSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let address_type = ExtendedAddressType::try_from(Read::by_ref(cursor))?;
        Ok(GetExtAddrSREQ { address_type })
    }
}

#[derive(Debug)]
pub struct GetExtAddrSRSP {
    pub address_type: ExtendedAddressType,
    pub ext_address: ExtendedAddress,
}

impl TryFrom<&mut Cursor<&[u8]>> for GetExtAddrSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let address_type = ExtendedAddressType::try_from(Read::by_ref(cursor))?;
        let ext_address = ExtendedAddress::try_from(Read::by_ref(cursor))?;
        Ok(GetExtAddrSRSP {
            address_type,
            ext_address,
        })
    }
}

#[derive(Debug)]
pub struct LoopbackSREQ {
    pub repeats: u8,
    pub interval: u32,
    pub data: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for LoopbackSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let repeats = cursor.get_u8();
        let interval = cursor.get_u32_le();

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(LoopbackSREQ {
            repeats,
            interval,
            data,
        })
    }
}

#[derive(Debug)]
pub struct LoopbackSRSP {
    pub repeats: u8,
    pub interval: u32,
    pub data: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for LoopbackSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let repeats = cursor.get_u8();
        let interval = cursor.get_u32_le();

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(LoopbackSRSP {
            repeats,
            interval,
            data,
        })
    }
}

#[derive(Debug)]
pub struct LoopbackAREQ {
    pub repeats: u8,
    pub interval: u32,
    pub data: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for LoopbackAREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let repeats = cursor.get_u8();
        let interval = cursor.get_u32_le();

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(LoopbackAREQ {
            repeats,
            interval,
            data,
        })
    }
}

#[derive(Debug)]
pub struct RandomSREQ {}

impl TryFrom<&mut Cursor<&[u8]>> for RandomSREQ {
    type Error = Error;
    fn try_from(_: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        Ok(RandomSREQ {})
    }
}

#[derive(Debug)]
pub struct RandomSRSP {
    pub number: u16,
}

impl TryFrom<&mut Cursor<&[u8]>> for RandomSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let number = cursor.get_u16_le();
        Ok(RandomSRSP { number })
    }
}
