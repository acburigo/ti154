use crate::error::Error;
use crate::types::*;
use bytes::Buf;
use std::io::Cursor;
use std::io::Read;

#[derive(Debug)]
pub struct Init {}

impl Init {
    pub fn try_from(_: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        Ok(Init {})
    }
}

#[derive(Debug)]
pub struct DataReq {
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

impl DataReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
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

        Ok(DataReq {
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
pub struct PurgeReq {
    pub handle: u8,
}

impl PurgeReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let handle = cursor.get_u8();
        Ok(PurgeReq { handle })
    }
}

#[derive(Debug)]
pub struct AssociateReq {
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

impl AssociateReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
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
        Ok(AssociateReq {
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
pub struct AssociateRsp {
    pub extended_address: ExtendedAddress,
    pub assoc_short_address: ShortAddress,
    pub assoc_status: AssociationStatus,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
}

impl AssociateRsp {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let extended_address = ExtendedAddress::try_from(Read::by_ref(cursor))?;
        let assoc_short_address = ShortAddress::try_from(Read::by_ref(cursor))?;
        let assoc_status = AssociationStatus::try_from(Read::by_ref(cursor))?;
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();
        Ok(AssociateRsp {
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
pub struct DisassociateReq {
    pub device_address: Address,
    pub device_pan_id: u16,
    pub disassociate_reason: DisassociateReason,
    pub tx_indirect: u8,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
}

impl DisassociateReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let device_address = Address::try_from(Read::by_ref(cursor))?;
        let device_pan_id = cursor.get_u16_le();
        let disassociate_reason = DisassociateReason::try_from(Read::by_ref(cursor))?;
        let tx_indirect = cursor.get_u8();
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();
        Ok(DisassociateReq {
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
pub struct GetReq {
    pub attribute_id: MACPIBAttributeId,
}

impl GetReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let attribute_id = MACPIBAttributeId::try_from(Read::by_ref(cursor))?;
        Ok(GetReq { attribute_id })
    }
}

#[derive(Debug)]
pub struct SetReq {
    pub attribute_id: MACPIBAttributeId,
    pub attribute_value: [u8; 16],
}

impl SetReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let attribute_id = MACPIBAttributeId::try_from(Read::by_ref(cursor))?;

        let mut attribute_value: [u8; 16] = Default::default();
        cursor
            .read_exact(&mut attribute_value)
            .map_err(|_| Error::NotEnoughBytes)?;
        attribute_value.reverse();

        Ok(SetReq {
            attribute_id,
            attribute_value,
        })
    }
}

#[derive(Debug)]
pub struct SecurityGetReq {
    pub attribute_id: SecurityPIBAttributeId,
    pub index1: u8,
    pub index2: u8,
}

impl SecurityGetReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let attribute_id = SecurityPIBAttributeId::try_from(Read::by_ref(cursor))?;
        let index1 = cursor.get_u8();
        let index2 = cursor.get_u8();

        Ok(SecurityGetReq {
            attribute_id,
            index1,
            index2,
        })
    }
}

#[derive(Debug)]
pub struct SecuritySetReq {
    pub attribute_id: SecurityPIBAttributeId,
    pub index1: u8,
    pub index2: u8,
    pub attribute_value: Vec<u8>,
}

impl SecuritySetReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let attribute_id = SecurityPIBAttributeId::try_from(Read::by_ref(cursor))?;
        let index1 = cursor.get_u8();
        let index2 = cursor.get_u8();

        let mut attribute_value = Vec::new();
        cursor
            .read_to_end(&mut attribute_value)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(SecuritySetReq {
            attribute_id,
            index1,
            index2,
            attribute_value,
        })
    }
}

#[derive(Debug)]
pub struct UpdatePANIdReq {
    pub pan_id: u16,
}

impl UpdatePANIdReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let pan_id = cursor.get_u16_le();
        Ok(UpdatePANIdReq { pan_id })
    }
}

#[derive(Debug)]
pub struct AddDeviceReq {
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

impl AddDeviceReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
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

        Ok(AddDeviceReq {
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
pub struct DeleteDeviceReq {
    pub ext_addr: ExtendedAddress,
}

impl DeleteDeviceReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let ext_addr = ExtendedAddress::try_from(Read::by_ref(cursor))?;
        Ok(DeleteDeviceReq { ext_addr })
    }
}

#[derive(Debug)]
pub struct DeleteAllDevicesReq {}

impl DeleteAllDevicesReq {
    pub fn try_from(_: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        Ok(DeleteAllDevicesReq {})
    }
}

#[derive(Debug)]
pub struct DeleteKeyReq {
    pub index: u8,
}

impl DeleteKeyReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let index = cursor.get_u8();
        Ok(DeleteKeyReq { index })
    }
}

#[derive(Debug)]
pub struct ReadKeyReq {
    pub index: u8,
}

impl ReadKeyReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let index = cursor.get_u8();
        Ok(ReadKeyReq { index })
    }
}

#[derive(Debug)]
pub struct WriteKeyReq {
    pub new: bool,
    pub index: u16,
    pub key: [u8; 16],
    pub frame_counter: u32,
    pub data_size: u8,
    pub lookup_data: [u8; 9],
}

impl WriteKeyReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
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

        Ok(WriteKeyReq {
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
pub struct OrphanRsp {
    pub extended_address: ExtendedAddress,
    pub assoc_short_address: ShortAddress,
    pub associated_member: bool,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
}

impl OrphanRsp {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let extended_address = ExtendedAddress::try_from(Read::by_ref(cursor))?;
        let assoc_short_address = ShortAddress::try_from(Read::by_ref(cursor))?;
        let associated_member = cursor.get_u8() != 0;
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();
        Ok(OrphanRsp {
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
pub struct PollReq {
    pub coord_address: Address,
    pub coord_pan_id: u16,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
}

impl PollReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let coord_address = Address::try_from(Read::by_ref(cursor))?;
        let coord_pan_id = cursor.get_u16_le();
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();
        Ok(PollReq {
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
pub struct ResetReq {
    pub set_default: bool,
}

impl ResetReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let set_default = cursor.get_u8() != 0;
        Ok(ResetReq { set_default })
    }
}

#[derive(Debug)]
pub struct ScanReq {
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
    pub channels: ChannelsBitMap,
}

impl ScanReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
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
        let channels = ChannelsBitMap::try_from(Read::by_ref(cursor))?;
        Ok(ScanReq {
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
pub struct StartReq {
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

impl StartReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
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

        Ok(StartReq {
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
pub struct SyncReq {
    pub logical_channel: u8,
    pub channel_page: u8,
    pub track_beacon: bool,
    pub phy_id: PhyId,
}

impl SyncReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let logical_channel = cursor.get_u8();
        let channel_page = cursor.get_u8();
        let track_beacon = cursor.get_u8() != 0;
        let phy_id = PhyId::try_from(Read::by_ref(cursor))?;
        Ok(SyncReq {
            logical_channel,
            channel_page,
            track_beacon,
            phy_id,
        })
    }
}

#[derive(Debug)]
pub struct SetRxGainReq {
    pub mode: bool,
}

impl SetRxGainReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let mode = cursor.get_u8() != 0;
        Ok(SetRxGainReq { mode })
    }
}

#[derive(Debug)]
pub struct WSAsyncReq {
    pub operation: WiSUNAsyncOperation,
    pub frame_type: WiSUNAsyncFrameType,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
    pub channels: ChannelsBitMap,
}

impl WSAsyncReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let operation = WiSUNAsyncOperation::try_from(Read::by_ref(cursor))?;
        let frame_type = WiSUNAsyncFrameType::try_from(Read::by_ref(cursor))?;
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();
        let channels = ChannelsBitMap::try_from(Read::by_ref(cursor))?;
        Ok(WSAsyncReq {
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
pub struct FHEnableReq {}

impl FHEnableReq {
    pub fn try_from(_: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        Ok(FHEnableReq {})
    }
}

#[derive(Debug)]
pub struct FHStartReq {}

impl FHStartReq {
    pub fn try_from(_: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        Ok(FHStartReq {})
    }
}

#[derive(Debug)]
pub struct FHGetReq {
    pub attribute_id: FHPIBAttributeId,
}

impl FHGetReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let attribute_id = FHPIBAttributeId::try_from(Read::by_ref(cursor))?;
        Ok(FHGetReq { attribute_id })
    }
}

#[derive(Debug)]
pub struct FHSetReq {
    pub attribute_id: FHPIBAttributeId,
    pub data: Vec<u8>,
}

impl FHSetReq {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let attribute_id = FHPIBAttributeId::try_from(Read::by_ref(cursor))?;

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(FHSetReq { attribute_id, data })
    }
}
