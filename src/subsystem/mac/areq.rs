use crate::error::Error;
use crate::types::*;
use bytes::Buf;
use std::io::Cursor;
use std::io::Read;

#[derive(Debug)]
pub struct DataCnf {
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

impl DataCnf {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let handle = cursor.get_u8();
        let timestamp = cursor.get_u32_le();
        let timestamp2 = cursor.get_u16_le();
        let retries = cursor.get_u8();
        let link_quality = cursor.get_u8();
        let correlation = cursor.get_u8();
        let rssi = cursor.get_u8();
        let frame_counter = cursor.get_u32_le();

        Ok(DataCnf {
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
pub struct DataInd {
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

impl DataInd {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
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

        Ok(DataInd {
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
pub struct PurgeCnf {
    pub status: Status,
    pub handle: u8,
}

impl PurgeCnf {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let handle = cursor.get_u8();
        Ok(PurgeCnf { status, handle })
    }
}

#[derive(Debug)]
pub struct WSAsyncInd {
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

impl WSAsyncInd {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
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

        Ok(WSAsyncInd {
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
pub struct SyncLossInd {
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

impl SyncLossInd {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let pan_id = cursor.get_u16_le();
        let logical_channel = cursor.get_u8();
        let channel_page = cursor.get_u8();
        let phy_id = PhyId::try_from(Read::by_ref(cursor))?;
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();

        Ok(SyncLossInd {
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
pub struct AssociateInd {
    pub extended_address: ExtendedAddress,
    pub capabilities: u8,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
}

impl AssociateInd {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let extended_address = ExtendedAddress::try_from(Read::by_ref(cursor))?;
        let capabilities = cursor.get_u8();
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();

        Ok(AssociateInd {
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
pub struct AssociateCnf {
    pub status: Status,
    pub short_address: ShortAddress,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
}

impl AssociateCnf {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let short_address = ShortAddress::try_from(Read::by_ref(cursor))?;
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();

        Ok(AssociateCnf {
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
pub enum BeaconNotifyInd {
    StandardFrame(StandardBeaconFrame),
    EnhancedFrame(EnhancedBeaconFrame),
}

impl BeaconNotifyInd {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        use BeaconNotifyInd::{EnhancedFrame, StandardFrame};

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

impl StandardBeaconFrame {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
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

impl EnhancedBeaconFrame {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
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
pub struct DisassociateInd {
    pub extended_address: ExtendedAddress,
    pub disassociate_reason: DisassociateReason,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
}

impl DisassociateInd {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let extended_address = ExtendedAddress::try_from(Read::by_ref(cursor))?;
        let disassociate_reason = DisassociateReason::try_from(Read::by_ref(cursor))?;
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();

        Ok(DisassociateInd {
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
pub struct DisassociateCnf {
    pub status: Status,
    pub device_addr: Address,
    pub device_pan_id: u16,
}

impl DisassociateCnf {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let device_addr = Address::try_from(Read::by_ref(cursor))?;
        let device_pan_id = cursor.get_u16_le();

        Ok(DisassociateCnf {
            status,
            device_addr,
            device_pan_id,
        })
    }
}

#[derive(Debug)]
pub struct OrphanInd {
    pub extended_address: ExtendedAddress,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
}

impl OrphanInd {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let extended_address = ExtendedAddress::try_from(Read::by_ref(cursor))?;
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();
        Ok(OrphanInd {
            extended_address,
            key_source,
            security_level,
            key_id_mode,
            key_index,
        })
    }
}

#[derive(Debug)]
pub struct PollCnf {
    pub status: Status,
    pub frame_pending: bool,
}

impl PollCnf {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let frame_pending = cursor.get_u8() != 0;
        Ok(PollCnf {
            status,
            frame_pending,
        })
    }
}

#[derive(Debug)]
pub struct PollInd {
    pub dev_addr: Address,
    pub pan_id: u16,
    pub no_response: bool,
}

impl PollInd {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let dev_addr = Address::try_from(Read::by_ref(cursor))?;
        let pan_id = cursor.get_u16_le();
        let no_response = cursor.get_u8() != 0;
        Ok(PollInd {
            dev_addr,
            pan_id,
            no_response,
        })
    }
}

#[derive(Debug)]
pub struct ScanCnf {
    pub status: Status,
    pub scan_type: ScanType,
    pub channel_page: u8,
    pub phy_id: PhyId,
    pub unscanned_channels: [u8; 17],
    pub result_list_count: u8,
    pub result_list: Vec<u8>,
}

impl ScanCnf {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
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

        Ok(ScanCnf {
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
pub struct CommStatusInd {
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

impl CommStatusInd {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        let src_addr = Address::try_from(Read::by_ref(cursor))?;
        let dst_addr = Address::try_from(Read::by_ref(cursor))?;
        let device_pan_id = cursor.get_u16_le();
        let reason = CommEventReason::try_from(Read::by_ref(cursor))?;
        let key_source = KeySource::try_from(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_from(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_from(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();

        Ok(CommStatusInd {
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
pub struct StartCnf {
    pub status: Status,
}

impl StartCnf {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(StartCnf { status })
    }
}

#[derive(Debug)]
pub struct WSAsyncCnf {
    pub status: Status,
}

impl WSAsyncCnf {
    pub fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_from(Read::by_ref(cursor))?;
        Ok(WSAsyncCnf { status })
    }
}
