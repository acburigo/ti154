use crate::error::Error;
use crate::types::*;
use bytes::Buf;
use std::convert::TryFrom;
use std::io::Cursor;
use std::io::Read;

#[derive(Debug)]
pub struct InitSREQ {}

impl TryFrom<&mut Cursor<&[u8]>> for InitSREQ {
    type Error = Error;
    fn try_from(_: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        Ok(InitSREQ {})
    }
}

#[derive(Debug)]
pub struct InitSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for InitSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(InitSRSP { status })
    }
}

#[derive(Debug)]
pub struct DataReqSREQ {
    pub dest_address: Address,
    pub dest_pan_id: u16,
    pub src_address_mode: AddressMode,
    pub handle: u8,
    pub tx_option: TxOption,
    pub channel: u8,
    pub power: u8,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
    pub include_fh_ies: u32,
    pub data_length: u16,
    pub ie_length: u16,
    pub data_payload: Vec<u8>,
    pub ie_payload: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for DataReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let dest_address = Address::try_from(Read::by_ref(cursor))?;
        let dest_pan_id = cursor.get_u16_le();
        let src_address_mode = AddressMode::try_from(Read::by_ref(cursor))?;
        let handle = cursor.get_u8();
        let tx_option = TxOption::try_from(Read::by_ref(cursor))?;
        let channel = cursor.get_u8();
        let power = cursor.get_u8();
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();
        let include_fh_ies = cursor.get_u32_le();
        let data_length = cursor.get_u16_le();
        let ie_length = cursor.get_u16_le();

        let mut data_payload = vec![0x00; data_length as usize];
        cursor
            .read_exact(&mut data_payload)
            .map_err(|_| Error::NotEnoughBytes)?;

        let mut ie_payload = vec![0x00; data_length as usize];
        cursor
            .read_exact(&mut ie_payload)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(DataReqSREQ {
            dest_address,
            dest_pan_id,
            src_address_mode,
            handle,
            tx_option,
            channel,
            power,
            key_source,
            security_level,
            key_id_mode,
            key_index,
            include_fh_ies,
            data_length,
            ie_length,
            data_payload,
            ie_payload,
        })
    }
}

#[derive(Debug)]
pub struct DataReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for DataReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(DataReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct PurgeReqSREQ {
    pub handle: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for PurgeReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let handle = cursor.get_u8();
        Ok(PurgeReqSREQ { handle })
    }
}

#[derive(Debug)]
pub struct PurgeReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for PurgeReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(PurgeReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct DataCnfAREQ {
    pub status: Status,
    pub handle: u8,
    pub timestamp: u32,
    pub timestamp2: u16,
    pub retries: u8,
    pub link_quality: u8,
    pub correlation: u8,
    pub rssi: u8,
    pub frame_counter: u32,
}

impl TryFrom<&mut Cursor<&[u8]>> for DataCnfAREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let handle = cursor.get_u8();
        let timestamp = cursor.get_u32_le();
        let timestamp2 = cursor.get_u16_le();
        let retries = cursor.get_u8();
        let link_quality = cursor.get_u8();
        let correlation = cursor.get_u8();
        let rssi = cursor.get_u8();
        let frame_counter = cursor.get_u32_le();

        Ok(DataCnfAREQ {
            status,
            handle,
            timestamp,
            timestamp2,
            retries,
            link_quality,
            correlation,
            rssi,
            frame_counter,
        })
    }
}

#[derive(Debug)]
pub struct DataIndAREQ {
    pub src_address: Address,
    pub dest_address: Address,
    pub timestamp: u32,
    pub timestamp2: u16,
    pub src_pan_id: u16,
    pub dest_pan_id: u16,
    pub link_quality: u8,
    pub correlation: u8,
    pub rssi: u8,
    pub dsn: u8,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
    pub frame_counter: u32,
    pub data_length: u16,
    pub ie_length: u16,
    pub data_payload: Vec<u8>,
    pub ie_payload: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for DataIndAREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let src_address = Address::try_from(Read::by_ref(cursor))?;
        let dest_address = Address::try_from(Read::by_ref(cursor))?;
        let timestamp = cursor.get_u32_le();
        let timestamp2 = cursor.get_u16_le();
        let src_pan_id = cursor.get_u16_le();
        let dest_pan_id = cursor.get_u16_le();
        let link_quality = cursor.get_u8();
        let correlation = cursor.get_u8();
        let rssi = cursor.get_u8();
        let dsn = cursor.get_u8();
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();
        let frame_counter = cursor.get_u32_le();
        let data_length = cursor.get_u16_le();
        let ie_length = cursor.get_u16_le();

        let mut data_payload = vec![0x00; data_length as usize];
        cursor
            .read_exact(&mut data_payload)
            .map_err(|_| Error::NotEnoughBytes)?;

        let mut ie_payload = vec![0x00; data_length as usize];
        cursor
            .read_exact(&mut ie_payload)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(DataIndAREQ {
            src_address,
            dest_address,
            timestamp,
            timestamp2,
            src_pan_id,
            dest_pan_id,
            link_quality,
            correlation,
            rssi,
            dsn,
            key_source,
            security_level,
            key_id_mode,
            key_index,
            frame_counter,
            data_length,
            ie_length,
            data_payload,
            ie_payload,
        })
    }
}

#[derive(Debug)]
pub struct PurgeCnfAREQ {
    pub status: Status,
    pub handle: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for PurgeCnfAREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let handle = cursor.get_u8();
        Ok(PurgeCnfAREQ { status, handle })
    }
}

#[derive(Debug)]
pub struct WSAsyncIndAREQ {
    pub src_address: Address,
    pub dest_address: Address,
    pub timestamp: u32,
    pub timestamp2: u16,
    pub src_pan_id: u16,
    pub dest_pan_id: u16,
    pub link_quality: u8,
    pub correlation: u8,
    pub rssi: u8,
    pub dsn: u8,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
    pub frame_counter: u32,
    pub frame_type: WiSUNAsyncFrameType,
    pub data_length: u16,
    pub ie_length: u16,
    pub data_payload: Vec<u8>,
    pub ie_payload: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for WSAsyncIndAREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let src_address = Address::try_from(Read::by_ref(cursor))?;
        let dest_address = Address::try_from(Read::by_ref(cursor))?;
        let timestamp = cursor.get_u32_le();
        let timestamp2 = cursor.get_u16_le();
        let src_pan_id = cursor.get_u16_le();
        let dest_pan_id = cursor.get_u16_le();
        let link_quality = cursor.get_u8();
        let correlation = cursor.get_u8();
        let rssi = cursor.get_u8();
        let dsn = cursor.get_u8();
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();
        let frame_counter = cursor.get_u32_le();
        let frame_type = WiSUNAsyncFrameType::try_from(Read::by_ref(cursor))?;
        let data_length = cursor.get_u16_le();
        let ie_length = cursor.get_u16_le();

        let mut data_payload = vec![0x00; data_length as usize];
        cursor
            .read_exact(&mut data_payload)
            .map_err(|_| Error::NotEnoughBytes)?;

        let mut ie_payload = vec![0x00; data_length as usize];
        cursor
            .read_exact(&mut ie_payload)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(WSAsyncIndAREQ {
            src_address,
            dest_address,
            timestamp,
            timestamp2,
            src_pan_id,
            dest_pan_id,
            link_quality,
            correlation,
            rssi,
            dsn,
            key_source,
            security_level,
            key_id_mode,
            key_index,
            frame_counter,
            frame_type,
            data_length,
            ie_length,
            data_payload,
            ie_payload,
        })
    }
}

#[derive(Debug)]
pub struct AssociateReqSREQ {
    pub logical_channel: u8,
    pub channel_page: u8,
    pub phy_id: u8,
    pub coord_address: Address,
    pub coord_pan_id: u16,
    pub capability_info: u8,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for AssociateReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let logical_channel = cursor.get_u8();
        let channel_page = cursor.get_u8();
        let phy_id = cursor.get_u8();
        let coord_address = Address::try_from(Read::by_ref(cursor))?;
        let coord_pan_id = cursor.get_u16_le();
        let capability_info = cursor.get_u8();
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();
        Ok(AssociateReqSREQ {
            logical_channel,
            channel_page,
            phy_id,
            coord_address,
            coord_pan_id,
            capability_info,
            key_source,
            security_level,
            key_id_mode,
            key_index,
        })
    }
}

#[derive(Debug)]
pub struct AssociateReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for AssociateReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(AssociateReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct AssociateRspSREQ {
    pub extended_address: ExtendedAddress,
    pub assoc_short_address: ShortAddress,
    pub assoc_status: AssociationStatus,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for AssociateRspSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let extended_address = ExtendedAddress::try_from(Read::by_ref(cursor))?;
        let assoc_short_address = ShortAddress::try_from(Read::by_ref(cursor))?;
        let assoc_status = AssociationStatus::try_from(Read::by_ref(cursor))?;
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();
        Ok(AssociateRspSREQ {
            extended_address,
            assoc_short_address,
            assoc_status,
            key_source,
            security_level,
            key_id_mode,
            key_index,
        })
    }
}

#[derive(Debug)]
pub struct AssociateRspSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for AssociateRspSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(AssociateRspSRSP { status })
    }
}

#[derive(Debug)]
pub struct DisassociateReqSREQ {
    pub device_address: Address,
    pub device_pan_id: u16,
    pub disassociate_reason: DisassociateReason,
    pub tx_indirect: u8,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for DisassociateReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let device_address = Address::try_from(Read::by_ref(cursor))?;
        let device_pan_id = cursor.get_u16_le();
        let disassociate_reason = DisassociateReason::try_from(Read::by_ref(cursor))?;
        let tx_indirect = cursor.get_u8();
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();
        Ok(DisassociateReqSREQ {
            device_address,
            device_pan_id,
            disassociate_reason,
            tx_indirect,
            key_source,
            security_level,
            key_id_mode,
            key_index,
        })
    }
}

#[derive(Debug)]
pub struct DisassociateReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for DisassociateReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(DisassociateReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct GetReqSREQ {
    pub attribute_id: MACPIBAttributeId,
}

impl TryFrom<&mut Cursor<&[u8]>> for GetReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let attribute_id = MACPIBAttributeId::try_from(Read::by_ref(cursor))?;
        Ok(GetReqSREQ { attribute_id })
    }
}

#[derive(Debug)]
pub struct GetReqSRSP {
    pub status: Status,
    pub data: [u8; 16],
}

impl TryFrom<&mut Cursor<&[u8]>> for GetReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;

        let mut data: [u8; 16] = Default::default();
        cursor
            .read_exact(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;
        data.reverse();

        Ok(GetReqSRSP { status, data })
    }
}

#[derive(Debug)]
pub struct SetReqSREQ {
    pub attribute_id: MACPIBAttributeId,
    pub attribute_value: [u8; 16],
}

impl TryFrom<&mut Cursor<&[u8]>> for SetReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let attribute_id = MACPIBAttributeId::try_from(Read::by_ref(cursor))?;

        let mut attribute_value: [u8; 16] = Default::default();
        cursor
            .read_exact(&mut attribute_value)
            .map_err(|_| Error::NotEnoughBytes)?;
        attribute_value.reverse();

        Ok(SetReqSREQ {
            attribute_id,
            attribute_value,
        })
    }
}

#[derive(Debug)]
pub struct SetReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for SetReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(SetReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct SecurityGetReqSREQ {
    pub attribute_id: SecurityPIBAttributeId,
    pub index1: u8,
    pub index2: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for SecurityGetReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let attribute_id = SecurityPIBAttributeId::try_from(Read::by_ref(cursor))?;
        let index1 = cursor.get_u8();
        let index2 = cursor.get_u8();

        Ok(SecurityGetReqSREQ {
            attribute_id,
            index1,
            index2,
        })
    }
}

#[derive(Debug)]
pub struct SecurityGetReqSRSP {
    pub status: Status,
    pub index1: u8,
    pub index2: u8,
    pub data: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for SecurityGetReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let index1 = cursor.get_u8();
        let index2 = cursor.get_u8();

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(SecurityGetReqSRSP {
            status,
            index1,
            index2,
            data,
        })
    }
}

#[derive(Debug)]
pub struct SecuritySetReqSREQ {
    pub attribute_id: SecurityPIBAttributeId,
    pub index1: u8,
    pub index2: u8,
    pub attribute_value: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for SecuritySetReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let attribute_id = SecurityPIBAttributeId::try_from(Read::by_ref(cursor))?;
        let index1 = cursor.get_u8();
        let index2 = cursor.get_u8();

        let mut attribute_value = Vec::new();
        cursor
            .read_to_end(&mut attribute_value)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(SecuritySetReqSREQ {
            attribute_id,
            index1,
            index2,
            attribute_value,
        })
    }
}

#[derive(Debug)]
pub struct SecuritySetReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for SecuritySetReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(SecuritySetReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct UpdatePANIdReqSREQ {
    pub pan_id: u16,
}

impl TryFrom<&mut Cursor<&[u8]>> for UpdatePANIdReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let pan_id = cursor.get_u16_le();
        Ok(UpdatePANIdReqSREQ { pan_id })
    }
}

#[derive(Debug)]
pub struct UpdatePANIdReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for UpdatePANIdReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(UpdatePANIdReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct AddDeviceReqSREQ {
    pub pan_id: u16,
    pub short_addr: ShortAddress,
    pub ext_addr: ExtendedAddress,
    pub frame_counter: u32,
    pub exempt: bool,
    pub unique: bool,
    pub duplicate: bool,
    pub data_size: u8,
    pub lookup_data: [u8; 9],
}

impl TryFrom<&mut Cursor<&[u8]>> for AddDeviceReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let pan_id = cursor.get_u16_le();
        let short_addr = ShortAddress::try_from(Read::by_ref(cursor))?;
        let ext_addr = ExtendedAddress::try_from(Read::by_ref(cursor))?;
        let frame_counter = cursor.get_u32_le();
        let exempt = cursor.get_u8() != 0;
        let unique = cursor.get_u8() != 0;
        let duplicate = cursor.get_u8() != 0;
        let data_size = cursor.get_u8();

        let mut lookup_data: [u8; 9] = Default::default();
        cursor
            .read_exact(&mut lookup_data)
            .map_err(|_| Error::NotEnoughBytes)?;
        lookup_data.reverse();

        Ok(AddDeviceReqSREQ {
            pan_id,
            short_addr,
            ext_addr,
            frame_counter,
            exempt,
            unique,
            duplicate,
            data_size,
            lookup_data,
        })
    }
}

#[derive(Debug)]
pub struct AddDeviceReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for AddDeviceReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(AddDeviceReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct DeleteDeviceReqSREQ {
    pub ext_addr: ExtendedAddress,
}

impl TryFrom<&mut Cursor<&[u8]>> for DeleteDeviceReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let ext_addr = ExtendedAddress::try_from(Read::by_ref(cursor))?;
        Ok(DeleteDeviceReqSREQ { ext_addr })
    }
}

#[derive(Debug)]
pub struct DeleteDeviceReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for DeleteDeviceReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(DeleteDeviceReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct DeleteAllDevicesReqSREQ {}

impl TryFrom<&mut Cursor<&[u8]>> for DeleteAllDevicesReqSREQ {
    type Error = Error;
    fn try_from(_: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        Ok(DeleteAllDevicesReqSREQ {})
    }
}

#[derive(Debug)]
pub struct DeleteAllDevicesReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for DeleteAllDevicesReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(DeleteAllDevicesReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct DeleteKeyReqSREQ {
    pub index: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for DeleteKeyReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let index = cursor.get_u8();
        Ok(DeleteKeyReqSREQ { index })
    }
}

#[derive(Debug)]
pub struct DeleteKeyReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for DeleteKeyReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(DeleteKeyReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct ReadKeyReqSREQ {
    pub index: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for ReadKeyReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let index = cursor.get_u8();
        Ok(ReadKeyReqSREQ { index })
    }
}

#[derive(Debug)]
pub struct ReadKeyReqSRSP {
    pub status: Status,
    pub frame_counter: u32,
}

impl TryFrom<&mut Cursor<&[u8]>> for ReadKeyReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let frame_counter = cursor.get_u32_le();
        Ok(ReadKeyReqSRSP {
            status,
            frame_counter,
        })
    }
}

#[derive(Debug)]
pub struct WriteKeyReqSREQ {
    pub new: bool,
    pub index: u16,
    pub key: [u8; 16],
    pub frame_counter: u32,
    pub data_size: u8,
    pub lookup_data: [u8; 9],
}

impl TryFrom<&mut Cursor<&[u8]>> for WriteKeyReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let new = cursor.get_u8() != 0;
        let index = cursor.get_u16_le();

        let mut key: [u8; 16] = Default::default();
        cursor
            .read_exact(&mut key)
            .map_err(|_| Error::NotEnoughBytes)?;
        key.reverse();

        let frame_counter = cursor.get_u32_le();
        let data_size = cursor.get_u8();

        let mut lookup_data: [u8; 9] = Default::default();
        cursor
            .read_exact(&mut lookup_data)
            .map_err(|_| Error::NotEnoughBytes)?;
        lookup_data.reverse();

        Ok(WriteKeyReqSREQ {
            new,
            index,
            key,
            frame_counter,
            data_size,
            lookup_data,
        })
    }
}

#[derive(Debug)]
pub struct WriteKeyReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for WriteKeyReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(WriteKeyReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct OrphanRspSREQ {
    pub extended_address: ExtendedAddress,
    pub assoc_short_address: ShortAddress,
    pub associated_member: bool,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for OrphanRspSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let extended_address = ExtendedAddress::try_from(Read::by_ref(cursor))?;
        let assoc_short_address = ShortAddress::try_from(Read::by_ref(cursor))?;
        let associated_member = cursor.get_u8() != 0;
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();
        Ok(OrphanRspSREQ {
            extended_address,
            assoc_short_address,
            associated_member,
            key_source,
            security_level,
            key_id_mode,
            key_index,
        })
    }
}

#[derive(Debug)]
pub struct OrphanRspSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for OrphanRspSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(OrphanRspSRSP { status })
    }
}

#[derive(Debug)]
pub struct PollReqSREQ {
    pub coord_address: Address,
    pub coord_pan_id: u16,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for PollReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let coord_address = Address::try_from(Read::by_ref(cursor))?;
        let coord_pan_id = cursor.get_u16_le();
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();
        Ok(PollReqSREQ {
            coord_address,
            coord_pan_id,
            key_source,
            security_level,
            key_id_mode,
            key_index,
        })
    }
}

#[derive(Debug)]
pub struct PollReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for PollReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(PollReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct ResetReqSREQ {
    pub set_default: bool,
}

impl TryFrom<&mut Cursor<&[u8]>> for ResetReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let set_default = cursor.get_u8() != 0;
        Ok(ResetReqSREQ { set_default })
    }
}

#[derive(Debug)]
pub struct ResetReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for ResetReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(ResetReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct ScanReqSREQ {
    pub scan_type: ScanType,
    pub scan_duration: u8,
    pub channel_page: u8,
    pub phy_id: PhyId,
    pub max_results: u8,
    pub permit_join: PermitJoin,
    pub link_quality: u8,
    pub rsp_filter: u8,
    pub mpm_scan: MPMScan,
    pub mpm_type: MPMType,
    pub mpm_duration: u16,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
    pub channels: [u8; 17],
}

impl TryFrom<&mut Cursor<&[u8]>> for ScanReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let scan_type = ScanType::try_from(Read::by_ref(cursor))?;
        let scan_duration = cursor.get_u8();
        let channel_page = cursor.get_u8();
        let phy_id = PhyId::try_from(Read::by_ref(cursor))?;
        let max_results = cursor.get_u8();
        let permit_join = PermitJoin::try_from(Read::by_ref(cursor))?;
        let link_quality = cursor.get_u8();
        let rsp_filter = cursor.get_u8();
        let mpm_scan = MPMScan::try_from(Read::by_ref(cursor))?;
        let mpm_type = MPMType::try_from(Read::by_ref(cursor))?;
        let mpm_duration = cursor.get_u16_le();
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();

        let mut channels: [u8; 17] = Default::default();
        cursor
            .read_exact(&mut channels)
            .map_err(|_| Error::NotEnoughBytes)?;
        channels.reverse();

        Ok(ScanReqSREQ {
            scan_type,
            scan_duration,
            channel_page,
            phy_id,
            max_results,
            permit_join,
            link_quality,
            rsp_filter,
            mpm_scan,
            mpm_type,
            mpm_duration,
            key_source,
            security_level,
            key_id_mode,
            key_index,
            channels,
        })
    }
}

#[derive(Debug)]
pub struct ScanReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for ScanReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(ScanReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct StartReqSREQ {
    pub start_time: u32,
    pub pan_id: u16,
    pub logical_channel: u8,
    pub channel_page: u8,
    pub phy_id: PhyId,
    pub beacon_order: u8,
    pub super_frame_order: u8,
    pub pan_coordinator: bool,
    pub battery_life_ext: bool,
    pub coord_realignment: bool,
    pub realign_key_source: KeySource,
    pub realign_security_level: SecurityLevel,
    pub realign_key_id_mode: KeyIdMode,
    pub realign_key_index: u8,
    pub beacon_key_source: KeySource,
    pub beacon_security_level: SecurityLevel,
    pub beacon_key_id_mode: KeyIdMode,
    pub beacon_key_index: u8,
    pub start_fh: bool,
    pub enh_beacon_order: u8,
    pub ofs_time_slot: u8,
    pub non_beacon_order: u16,
    pub num_ies: u8,
    pub ie_id_list: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for StartReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let start_time = cursor.get_u32_le();
        let pan_id = cursor.get_u16_le();
        let logical_channel = cursor.get_u8();
        let channel_page = cursor.get_u8();
        let phy_id = PhyId::try_from(Read::by_ref(cursor))?;
        let beacon_order = cursor.get_u8();
        let super_frame_order = cursor.get_u8();
        let pan_coordinator = cursor.get_u8() != 0;
        let battery_life_ext = cursor.get_u8() != 0;
        let coord_realignment = cursor.get_u8() != 0;
        let realign_key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let realign_security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let realign_key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let realign_key_index = cursor.get_u8();
        let beacon_key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let beacon_security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let beacon_key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let beacon_key_index = cursor.get_u8();
        let start_fh = cursor.get_u8() != 0;
        let enh_beacon_order = cursor.get_u8();
        let ofs_time_slot = cursor.get_u8();
        let non_beacon_order = cursor.get_u16_le();
        let num_ies = cursor.get_u8();

        let mut ie_id_list = vec![0x00; num_ies as usize];
        cursor
            .read_exact(&mut ie_id_list)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(StartReqSREQ {
            start_time,
            pan_id,
            logical_channel,
            channel_page,
            phy_id,
            beacon_order,
            super_frame_order,
            pan_coordinator,
            battery_life_ext,
            coord_realignment,
            realign_key_source,
            realign_security_level,
            realign_key_id_mode,
            realign_key_index,
            beacon_key_source,
            beacon_security_level,
            beacon_key_id_mode,
            beacon_key_index,
            start_fh,
            enh_beacon_order,
            ofs_time_slot,
            non_beacon_order,
            num_ies,
            ie_id_list,
        })
    }
}

#[derive(Debug)]
pub struct StartReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for StartReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(StartReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct SyncReqSREQ {
    pub logical_channel: u8,
    pub channel_page: u8,
    pub track_beacon: bool,
    pub phy_id: PhyId,
}

impl TryFrom<&mut Cursor<&[u8]>> for SyncReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let logical_channel = cursor.get_u8();
        let channel_page = cursor.get_u8();
        let track_beacon = cursor.get_u8() != 0;
        let phy_id = PhyId::try_from(Read::by_ref(cursor))?;
        Ok(SyncReqSREQ {
            logical_channel,
            channel_page,
            track_beacon,
            phy_id,
        })
    }
}

#[derive(Debug)]
pub struct SyncReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for SyncReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(SyncReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct SetRxGainReqSREQ {
    pub mode: bool,
}

impl TryFrom<&mut Cursor<&[u8]>> for SetRxGainReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let mode = cursor.get_u8() != 0;
        Ok(SetRxGainReqSREQ { mode })
    }
}

#[derive(Debug)]
pub struct SetRxGainReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for SetRxGainReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(SetRxGainReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct WSAsyncReqSREQ {
    pub operation: WiSUNAsyncOperation,
    pub frame_type: WiSUNAsyncFrameType,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
    pub channels: [u8; 17],
}

impl TryFrom<&mut Cursor<&[u8]>> for WSAsyncReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let operation = WiSUNAsyncOperation::try_from(Read::by_ref(cursor))?;
        let frame_type = WiSUNAsyncFrameType::try_from(Read::by_ref(cursor))?;
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();

        let mut channels: [u8; 17] = Default::default();
        cursor
            .read_exact(&mut channels)
            .map_err(|_| Error::NotEnoughBytes)?;
        channels.reverse();

        Ok(WSAsyncReqSREQ {
            operation,
            frame_type,
            key_source,
            security_level,
            key_id_mode,
            key_index,
            channels,
        })
    }
}

#[derive(Debug)]
pub struct WSAsyncReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for WSAsyncReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(WSAsyncReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct FHEnableReqSREQ {}

impl TryFrom<&mut Cursor<&[u8]>> for FHEnableReqSREQ {
    type Error = Error;
    fn try_from(_: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        Ok(FHEnableReqSREQ {})
    }
}

#[derive(Debug)]
pub struct FHEnableReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for FHEnableReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(FHEnableReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct FHStartReqSREQ {}

impl TryFrom<&mut Cursor<&[u8]>> for FHStartReqSREQ {
    type Error = Error;
    fn try_from(_: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        Ok(FHStartReqSREQ {})
    }
}

#[derive(Debug)]
pub struct FHStartReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for FHStartReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(FHStartReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct FHGetReqSREQ {
    pub attribute_id: FHPIBAttributeId,
}

impl TryFrom<&mut Cursor<&[u8]>> for FHGetReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let attribute_id = FHPIBAttributeId::try_from(Read::by_ref(cursor))?;
        Ok(FHGetReqSREQ { attribute_id })
    }
}

#[derive(Debug)]
pub struct FHGetReqSRSP {
    pub status: Status,
    pub data: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for FHGetReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(FHGetReqSRSP { status, data })
    }
}

#[derive(Debug)]
pub struct FHSetReqSREQ {
    pub attribute_id: FHPIBAttributeId,
    pub data: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for FHSetReqSREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let attribute_id = FHPIBAttributeId::try_from(Read::by_ref(cursor))?;

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(FHSetReqSREQ { attribute_id, data })
    }
}

#[derive(Debug)]
pub struct FHSetReqSRSP {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for FHSetReqSRSP {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(FHSetReqSRSP { status })
    }
}

#[derive(Debug)]
pub struct SyncLossIndAREQ {
    pub status: Status,
    pub pan_id: u16,
    pub logical_channel: u8,
    pub channel_page: u8,
    pub phy_id: PhyId,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for SyncLossIndAREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let pan_id = cursor.get_u16_le();
        let logical_channel = cursor.get_u8();
        let channel_page = cursor.get_u8();
        let phy_id = PhyId::try_from(Read::by_ref(cursor))?;
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();

        Ok(SyncLossIndAREQ {
            status,
            pan_id,
            logical_channel,
            channel_page,
            phy_id,
            key_source,
            security_level,
            key_id_mode,
            key_index,
        })
    }
}

#[derive(Debug)]
pub struct AssociateIndAREQ {
    pub extended_address: ExtendedAddress,
    pub capabilities: u8,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for AssociateIndAREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let extended_address = ExtendedAddress::try_from(Read::by_ref(cursor))?;
        let capabilities = cursor.get_u8();
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();

        Ok(AssociateIndAREQ {
            extended_address,
            capabilities,
            key_source,
            security_level,
            key_id_mode,
            key_index,
        })
    }
}

#[derive(Debug)]
pub struct AssociateCnfAREQ {
    pub status: Status,
    pub short_address: ShortAddress,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for AssociateCnfAREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let short_address = ShortAddress::try_from(Read::by_ref(cursor))?;
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();

        Ok(AssociateCnfAREQ {
            status,
            short_address,
            key_source,
            security_level,
            key_id_mode,
            key_index,
        })
    }
}

#[derive(Debug)]
pub enum BeaconNotifyIndAREQ {
    StandardFrame(StandardBeaconFrame),
    EnhancedFrame(EnhancedBeaconFrame),
}

impl TryFrom<&mut Cursor<&[u8]>> for BeaconNotifyIndAREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        use BeaconNotifyIndAREQ::{EnhancedFrame, StandardFrame};

        let beacon_type = cursor.get_u8();

        let beacon_frame = match beacon_type {
            0 => StandardFrame(StandardBeaconFrame::try_from(Read::by_ref(cursor))?),
            1 => EnhancedFrame(EnhancedBeaconFrame::try_from(Read::by_ref(cursor))?),
            _ => return Err(Error::InvalidBeaconType(beacon_type)),
        };

        Ok(beacon_frame)
    }
}

#[derive(Debug)]
pub struct StandardBeaconFrame {
    pub bsn: u8,
    pub timestamp: u32,
    pub coord_address_mode: AddressMode,
    pub coord_extended_address: ExtendedAddress,
    pub pan_id: u16,
    pub superframe_spec: u16,
    pub logical_channel: u8,
    pub channel_page: u8,
    pub gts_permit: bool,
    pub link_quality: u8,
    pub security_failure: bool,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
    pub short_addrs: u8,
    pub ext_addrs: u8,
    pub sdu_length: u8,
    pub short_addr_list: Vec<ShortAddress>,
    pub ext_addr_list: Vec<ExtendedAddress>,
    pub nsdu: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for StandardBeaconFrame {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let bsn = cursor.get_u8();
        let timestamp = cursor.get_u32_le();
        let coord_address_mode = AddressMode::try_from(Read::by_ref(cursor))?;
        let coord_extended_address = ExtendedAddress::try_from(Read::by_ref(cursor))?;
        let pan_id = cursor.get_u16_le();
        let superframe_spec = cursor.get_u16_le();
        let logical_channel = cursor.get_u8();
        let channel_page = cursor.get_u8();
        let gts_permit = cursor.get_u8() != 0;
        let link_quality = cursor.get_u8();
        let security_failure = cursor.get_u8() != 0;
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();
        let short_addrs = cursor.get_u8();
        let ext_addrs = cursor.get_u8();
        let sdu_length = cursor.get_u8();

        let mut short_addr_list = Vec::new();
        for _ in 0..short_addrs {
            short_addr_list.push(ShortAddress::try_from(Read::by_ref(cursor))?);
        }

        let mut ext_addr_list = Vec::new();
        for _ in 0..ext_addrs {
            ext_addr_list.push(ExtendedAddress::try_from(Read::by_ref(cursor))?);
        }

        let mut nsdu = vec![0x00; sdu_length as usize];
        cursor
            .read_exact(&mut nsdu)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(StandardBeaconFrame {
            bsn,
            timestamp,
            coord_address_mode,
            coord_extended_address,
            pan_id,
            superframe_spec,
            logical_channel,
            channel_page,
            gts_permit,
            link_quality,
            security_failure,
            key_source,
            security_level,
            key_id_mode,
            key_index,
            short_addrs,
            ext_addrs,
            sdu_length,
            short_addr_list,
            ext_addr_list,
            nsdu,
        })
    }
}

#[derive(Debug)]
pub struct EnhancedBeaconFrame {
    pub bsn: u8,
    pub beacon_order: u8,
    pub super_frame_order: u8,
    pub final_cap_slot: u8,
    pub enh_beacon_order: u8,
    pub ofs_time_slot: u8,
    pub cap_back_off: u8,
    pub non_beacon_order: u16,
}

impl TryFrom<&mut Cursor<&[u8]>> for EnhancedBeaconFrame {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let bsn = cursor.get_u8();
        let beacon_order = cursor.get_u8();
        let super_frame_order = cursor.get_u8();
        let final_cap_slot = cursor.get_u8();
        let enh_beacon_order = cursor.get_u8();
        let ofs_time_slot = cursor.get_u8();
        let cap_back_off = cursor.get_u8();
        let non_beacon_order = cursor.get_u16_le();
        Ok(EnhancedBeaconFrame {
            bsn,
            beacon_order,
            super_frame_order,
            final_cap_slot,
            enh_beacon_order,
            ofs_time_slot,
            cap_back_off,
            non_beacon_order,
        })
    }
}

#[derive(Debug)]
pub struct DisassociateIndAREQ {
    pub extended_address: ExtendedAddress,
    pub disassociate_reason: DisassociateReason,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for DisassociateIndAREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let extended_address = ExtendedAddress::try_from(Read::by_ref(cursor))?;
        let disassociate_reason = DisassociateReason::try_from(Read::by_ref(cursor))?;
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();

        Ok(DisassociateIndAREQ {
            extended_address,
            disassociate_reason,
            key_source,
            security_level,
            key_id_mode,
            key_index,
        })
    }
}

#[derive(Debug)]
pub struct DisassociateCnfAREQ {
    pub status: Status,
    pub device_addr: Address,
    pub device_pan_id: u16,
}

impl TryFrom<&mut Cursor<&[u8]>> for DisassociateCnfAREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let device_addr = Address::try_from(Read::by_ref(cursor))?;
        let device_pan_id = cursor.get_u16_le();

        Ok(DisassociateCnfAREQ {
            status,
            device_addr,
            device_pan_id,
        })
    }
}

#[derive(Debug)]
pub struct OrphanIndAREQ {
    pub extended_address: ExtendedAddress,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for OrphanIndAREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let extended_address = ExtendedAddress::try_from(Read::by_ref(cursor))?;
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();
        Ok(OrphanIndAREQ {
            extended_address,
            key_source,
            security_level,
            key_id_mode,
            key_index,
        })
    }
}

#[derive(Debug)]
pub struct PollCnfAREQ {
    pub status: Status,
    pub frame_pending: bool,
}

impl TryFrom<&mut Cursor<&[u8]>> for PollCnfAREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let frame_pending = cursor.get_u8() != 0;
        Ok(PollCnfAREQ {
            status,
            frame_pending,
        })
    }
}

#[derive(Debug)]
pub struct PollIndAREQ {
    pub dev_addr: Address,
    pub pan_id: u16,
    pub no_response: bool,
}

impl TryFrom<&mut Cursor<&[u8]>> for PollIndAREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let dev_addr = Address::try_from(Read::by_ref(cursor))?;
        let pan_id = cursor.get_u16_le();
        let no_response = cursor.get_u8() != 0;
        Ok(PollIndAREQ {
            dev_addr,
            pan_id,
            no_response,
        })
    }
}

#[derive(Debug)]
pub struct ScanCnfAREQ {
    pub status: Status,
    pub scan_type: ScanType,
    pub channel_page: u8,
    pub phy_id: PhyId,
    pub unscanned_channels: [u8; 17],
    pub result_list_count: u8,
    pub result_list: Vec<u8>,
}

impl TryFrom<&mut Cursor<&[u8]>> for ScanCnfAREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let scan_type = ScanType::try_from(Read::by_ref(cursor))?;
        let channel_page = cursor.get_u8();
        let phy_id = PhyId::try_from(Read::by_ref(cursor))?;

        let mut unscanned_channels: [u8; 17] = Default::default();
        cursor
            .read_exact(&mut unscanned_channels)
            .map_err(|_| Error::NotEnoughBytes)?;
        unscanned_channels.reverse();

        let result_list_count = cursor.get_u8();

        let mut result_list = Vec::new();
        cursor
            .read_to_end(&mut result_list)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(ScanCnfAREQ {
            status,
            scan_type,
            channel_page,
            phy_id,
            unscanned_channels,
            result_list_count,
            result_list,
        })
    }
}

#[derive(Debug)]
pub struct CommStatusIndAREQ {
    pub status: Status,
    pub src_addr: Address,
    pub dst_addr: Address,
    pub device_pan_id: u16,
    pub reason: CommEventReason,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for CommStatusIndAREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let src_addr = Address::try_from(Read::by_ref(cursor))?;
        let dst_addr = Address::try_from(Read::by_ref(cursor))?;
        let device_pan_id = cursor.get_u16_le();
        let reason = CommEventReason::try_from(Read::by_ref(cursor))?;
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();

        Ok(CommStatusIndAREQ {
            status,
            src_addr,
            dst_addr,
            device_pan_id,
            reason,
            key_source,
            security_level,
            key_id_mode,
            key_index,
        })
    }
}

#[derive(Debug)]
pub struct StartCnfAREQ {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for StartCnfAREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(StartCnfAREQ { status })
    }
}

#[derive(Debug)]
pub struct WSAsyncCnfAREQ {
    pub status: Status,
}

impl TryFrom<&mut Cursor<&[u8]>> for WSAsyncCnfAREQ {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(WSAsyncCnfAREQ { status })
    }
}
