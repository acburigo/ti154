use crate::error::Error;
use crate::frame::{CommandCode, MTFrame, MTHeader};
use crate::subsystem::MTFramePayload;
use crate::types::*;
use bytes::{Buf, BufMut};
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
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
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

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.status.encode_into(buffer);
        buffer.put_u8(self.handle);
        buffer.put_u32_le(self.timestamp);
        buffer.put_u16_le(self.timestamp2);
        buffer.put_u8(self.retries);
        buffer.put_u8(self.link_quality);
        buffer.put_u8(self.correlation);
        buffer.put_u8(self.rssi);
        buffer.put_u32_le(self.frame_counter);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x10,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::AREQ,
                    subsystem: MTSubsystem::MAC,
                    id: 0x84,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_DataCnf_AREQ(self),
        }
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
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let src_address = Address::try_decode(Read::by_ref(cursor))?;
        let dest_address = Address::try_decode(Read::by_ref(cursor))?;
        let timestamp = cursor.get_u32_le();
        let timestamp2 = cursor.get_u16_le();
        let src_pan_id = cursor.get_u16_le();
        let dest_pan_id = cursor.get_u16_le();
        let link_quality = cursor.get_u8();
        let correlation = cursor.get_u8();
        let rssi = cursor.get_u8();
        let dsn = cursor.get_u8();
        let key_source = KeySource::try_decode(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_decode(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_decode(Read::by_ref(cursor))?;
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

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.src_address.encode_into(buffer);
        self.dest_address.encode_into(buffer);
        buffer.put_u32_le(self.timestamp);
        buffer.put_u16_le(self.timestamp2);
        buffer.put_u16_le(self.src_pan_id);
        buffer.put_u16_le(self.dest_pan_id);
        buffer.put_u8(self.link_quality);
        buffer.put_u8(self.correlation);
        buffer.put_u8(self.rssi);
        buffer.put_u8(self.dsn);
        self.key_source.encode_into(buffer);
        self.security_level.encode_into(buffer);
        self.key_id_mode.encode_into(buffer);
        buffer.put_u8(self.key_index);
        buffer.put_u32_le(self.frame_counter);
        buffer.put_u16_le(self.data_length);
        buffer.put_u16_le(self.ie_length);
        buffer.extend(self.data_payload.iter());
        buffer.extend(self.ie_payload.iter());
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: (0x33 + self.data_payload.len() + self.ie_payload.len()) as u8,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::AREQ,
                    subsystem: MTSubsystem::MAC,
                    id: 0x85,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_DataInd_AREQ(self),
        }
    }
}

#[derive(Debug)]
pub struct PurgeCnf {
    pub status: Status,
    pub handle: u8,
}

impl PurgeCnf {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        let handle = cursor.get_u8();
        Ok(PurgeCnf { status, handle })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.status.encode_into(buffer);
        buffer.put_u8(self.handle);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x02,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::AREQ,
                    subsystem: MTSubsystem::MAC,
                    id: 0x90,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_PurgeCnf_AREQ(self),
        }
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
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let src_address = Address::try_decode(Read::by_ref(cursor))?;
        let dest_address = Address::try_decode(Read::by_ref(cursor))?;
        let timestamp = cursor.get_u32_le();
        let timestamp2 = cursor.get_u16_le();
        let src_pan_id = cursor.get_u16_le();
        let dest_pan_id = cursor.get_u16_le();
        let link_quality = cursor.get_u8();
        let correlation = cursor.get_u8();
        let rssi = cursor.get_u8();
        let dsn = cursor.get_u8();
        let key_source = KeySource::try_decode(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_decode(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_decode(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();
        let frame_counter = cursor.get_u32_le();
        let frame_type = WiSUNAsyncFrameType::try_decode(Read::by_ref(cursor))?;
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

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.src_address.encode_into(buffer);
        self.dest_address.encode_into(buffer);
        buffer.put_u32_le(self.timestamp);
        buffer.put_u16_le(self.timestamp2);
        buffer.put_u16_le(self.src_pan_id);
        buffer.put_u16_le(self.dest_pan_id);
        buffer.put_u8(self.link_quality);
        buffer.put_u8(self.correlation);
        buffer.put_u8(self.rssi);
        buffer.put_u8(self.dsn);
        self.key_source.encode_into(buffer);
        self.security_level.encode_into(buffer);
        self.key_id_mode.encode_into(buffer);
        buffer.put_u8(self.key_index);
        buffer.put_u32_le(self.frame_counter);
        self.frame_type.encode_into(buffer);
        buffer.put_u16_le(self.data_length);
        buffer.put_u16_le(self.ie_length);
        buffer.extend(self.data_payload.iter());
        buffer.extend(self.ie_payload.iter());
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: (0x34 + self.data_payload.len() + self.ie_payload.len()) as u8,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::AREQ,
                    subsystem: MTSubsystem::MAC,
                    id: 0x93,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_WSAsyncInd_AREQ(self),
        }
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
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        let pan_id = cursor.get_u16_le();
        let logical_channel = cursor.get_u8();
        let channel_page = cursor.get_u8();
        let phy_id = PhyId::try_decode(Read::by_ref(cursor))?;
        let key_source = KeySource::try_decode(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_decode(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_decode(Read::by_ref(cursor))?;
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

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.status.encode_into(buffer);
        buffer.put_u16_le(self.pan_id);
        buffer.put_u8(self.logical_channel);
        buffer.put_u8(self.channel_page);
        self.phy_id.encode_into(buffer);
        self.key_source.encode_into(buffer);
        self.security_level.encode_into(buffer);
        self.key_id_mode.encode_into(buffer);
        buffer.put_u8(self.key_index);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x11,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::AREQ,
                    subsystem: MTSubsystem::MAC,
                    id: 0x80,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_SyncLossInd_AREQ(self),
        }
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
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let extended_address = ExtendedAddress::try_decode(Read::by_ref(cursor))?;
        let capabilities = cursor.get_u8();
        let key_source = KeySource::try_decode(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_decode(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_decode(Read::by_ref(cursor))?;
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

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.extended_address.encode_into(buffer);
        buffer.put_u8(self.capabilities);
        self.key_source.encode_into(buffer);
        self.security_level.encode_into(buffer);
        self.key_id_mode.encode_into(buffer);
        buffer.put_u8(self.key_index);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x14,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::AREQ,
                    subsystem: MTSubsystem::MAC,
                    id: 0x81,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_AssociateInd_AREQ(self),
        }
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
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        let short_address = ShortAddress::try_decode(Read::by_ref(cursor))?;
        let key_source = KeySource::try_decode(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_decode(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_decode(Read::by_ref(cursor))?;
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

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.status.encode_into(buffer);
        self.short_address.encode_into(buffer);
        self.key_source.encode_into(buffer);
        self.security_level.encode_into(buffer);
        self.key_id_mode.encode_into(buffer);
        buffer.put_u8(self.key_index);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x0e,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::AREQ,
                    subsystem: MTSubsystem::MAC,
                    id: 0x82,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_AssociateCnf_AREQ(self),
        }
    }
}

#[derive(Debug)]
pub enum BeaconNotifyInd {
    StandardFrame(StandardBeaconFrame),
    EnhancedFrame(EnhancedBeaconFrame),
}

impl BeaconNotifyInd {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        use BeaconNotifyInd::{EnhancedFrame, StandardFrame};

        let beacon_type = cursor.get_u8();

        let beacon_frame = match beacon_type {
            0 => StandardFrame(StandardBeaconFrame::try_decode(Read::by_ref(cursor))?),
            1 => EnhancedFrame(EnhancedBeaconFrame::try_decode(Read::by_ref(cursor))?),
            _ => return Err(Error::InvalidBeaconType(beacon_type)),
        };

        Ok(beacon_frame)
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        match self {
            BeaconNotifyInd::StandardFrame(frame) => {
                buffer.put_u8(0);
                frame.encode_into(buffer);
            }
            BeaconNotifyInd::EnhancedFrame(frame) => {
                buffer.put_u8(1);
                frame.encode_into(buffer);
            }
        }
    }

    pub fn into_mt_frame(self) -> MTFrame {
        use BeaconNotifyInd::*;
        let length = match self {
            StandardFrame(ref frame) => {
                0x26 + (2 * frame.short_addr_list.len() + 8 * frame.ext_addr_list.len()) as u8
                    + frame.sdu_length
            }
            EnhancedFrame(_) => 0x0a,
        };
        MTFrame {
            header: MTHeader {
                length,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::AREQ,
                    subsystem: MTSubsystem::MAC,
                    id: 0x83,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_BeaconNotifyInd_AREQ(self),
        }
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
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let bsn = cursor.get_u8();
        let timestamp = cursor.get_u32_le();
        let coord_address_mode = AddressMode::try_decode(Read::by_ref(cursor))?;
        let coord_extended_address = ExtendedAddress::try_decode(Read::by_ref(cursor))?;
        let pan_id = cursor.get_u16_le();
        let superframe_spec = cursor.get_u16_le();
        let logical_channel = cursor.get_u8();
        let channel_page = cursor.get_u8();
        let gts_permit = cursor.get_u8() != 0;
        let link_quality = cursor.get_u8();
        let security_failure = cursor.get_u8() != 0;
        let key_source = KeySource::try_decode(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_decode(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_decode(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();
        let short_addrs = cursor.get_u8();
        let ext_addrs = cursor.get_u8();
        let sdu_length = cursor.get_u8();

        let mut short_addr_list = Vec::new();
        for _ in 0..short_addrs {
            short_addr_list.push(ShortAddress::try_decode(Read::by_ref(cursor))?);
        }

        let mut ext_addr_list = Vec::new();
        for _ in 0..ext_addrs {
            ext_addr_list.push(ExtendedAddress::try_decode(Read::by_ref(cursor))?);
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

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(self.bsn);
        buffer.put_u32_le(self.timestamp);
        self.coord_address_mode.encode_into(buffer);
        self.coord_extended_address.encode_into(buffer);
        buffer.put_u16_le(self.pan_id);
        buffer.put_u16_le(self.superframe_spec);
        buffer.put_u8(self.logical_channel);
        buffer.put_u8(self.channel_page);
        buffer.put_u8(if self.gts_permit { 1 } else { 0 });
        buffer.put_u8(self.link_quality);
        buffer.put_u8(if self.security_failure { 1 } else { 0 });
        self.key_source.encode_into(buffer);
        self.security_level.encode_into(buffer);
        self.key_id_mode.encode_into(buffer);
        buffer.put_u8(self.key_index);
        buffer.put_u8(self.short_addrs);
        buffer.put_u8(self.ext_addrs);
        buffer.put_u8(self.sdu_length);

        for short_address in self.short_addr_list.iter() {
            short_address.encode_into(buffer);
        }

        for extended_address in self.ext_addr_list.iter() {
            extended_address.encode_into(buffer);
        }

        buffer.extend(self.nsdu.iter());
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
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
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

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(self.bsn);
        buffer.put_u8(self.beacon_order);
        buffer.put_u8(self.super_frame_order);
        buffer.put_u8(self.final_cap_slot);
        buffer.put_u8(self.enh_beacon_order);
        buffer.put_u8(self.ofs_time_slot);
        buffer.put_u8(self.cap_back_off);
        buffer.put_u16_le(self.non_beacon_order);
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
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let extended_address = ExtendedAddress::try_decode(Read::by_ref(cursor))?;
        let disassociate_reason = DisassociateReason::try_decode(Read::by_ref(cursor))?;
        let key_source = KeySource::try_decode(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_decode(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_decode(Read::by_ref(cursor))?;
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

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.extended_address.encode_into(buffer);
        self.disassociate_reason.encode_into(buffer);
        self.key_source.encode_into(buffer);
        self.security_level.encode_into(buffer);
        self.key_id_mode.encode_into(buffer);
        buffer.put_u8(self.key_index);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x14,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::AREQ,
                    subsystem: MTSubsystem::MAC,
                    id: 0x86,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_DisassociateInd_AREQ(self),
        }
    }
}

#[derive(Debug)]
pub struct DisassociateCnf {
    pub status: Status,
    pub device_addr: Address,
    pub device_pan_id: u16,
}

impl DisassociateCnf {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        let device_addr = Address::try_decode(Read::by_ref(cursor))?;
        let device_pan_id = cursor.get_u16_le();

        Ok(DisassociateCnf {
            status,
            device_addr,
            device_pan_id,
        })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.status.encode_into(buffer);
        self.device_addr.encode_into(buffer);
        buffer.put_u16_le(self.device_pan_id);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x0c,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::AREQ,
                    subsystem: MTSubsystem::MAC,
                    id: 0x87,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_DisassociateCnf_AREQ(self),
        }
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
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let extended_address = ExtendedAddress::try_decode(Read::by_ref(cursor))?;
        let key_source = KeySource::try_decode(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_decode(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_decode(Read::by_ref(cursor))?;
        let key_index = cursor.get_u8();
        Ok(OrphanInd {
            extended_address,
            key_source,
            security_level,
            key_id_mode,
            key_index,
        })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.extended_address.encode_into(buffer);
        self.key_source.encode_into(buffer);
        self.security_level.encode_into(buffer);
        self.key_id_mode.encode_into(buffer);
        buffer.put_u8(self.key_index);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x13,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::AREQ,
                    subsystem: MTSubsystem::MAC,
                    id: 0x8a,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_OrphanInd_AREQ(self),
        }
    }
}

#[derive(Debug)]
pub struct PollCnf {
    pub status: Status,
    pub frame_pending: bool,
}

impl PollCnf {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        let frame_pending = cursor.get_u8() != 0;
        Ok(PollCnf {
            status,
            frame_pending,
        })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.status.encode_into(buffer);
        buffer.put_u8(if self.frame_pending { 1 } else { 0 });
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x02,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::AREQ,
                    subsystem: MTSubsystem::MAC,
                    id: 0x8b,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_PollCnf_AREQ(self),
        }
    }
}

#[derive(Debug)]
pub struct PollInd {
    pub dev_addr: Address,
    pub pan_id: u16,
    pub no_response: bool,
}

impl PollInd {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let dev_addr = Address::try_decode(Read::by_ref(cursor))?;
        let pan_id = cursor.get_u16_le();
        let no_response = cursor.get_u8() != 0;
        Ok(PollInd {
            dev_addr,
            pan_id,
            no_response,
        })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.dev_addr.encode_into(buffer);
        buffer.put_u16_le(self.pan_id);
        buffer.put_u8(if self.no_response { 1 } else { 0 });
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x0c,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::AREQ,
                    subsystem: MTSubsystem::MAC,
                    id: 0x91,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_PollInd_AREQ(self),
        }
    }
}

#[derive(Debug)]
pub struct ScanCnf {
    pub status: Status,
    pub scan_type: ScanType,
    pub channel_page: u8,
    pub phy_id: PhyId,
    pub unscanned_channels: ChannelsBitMap,
    pub result_list_count: u8,
    pub result_list: Vec<u8>,
}

impl ScanCnf {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        let scan_type = ScanType::try_decode(Read::by_ref(cursor))?;
        let channel_page = cursor.get_u8();
        let phy_id = PhyId::try_decode(Read::by_ref(cursor))?;
        let unscanned_channels = ChannelsBitMap::try_decode(Read::by_ref(cursor))?;
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

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.status.encode_into(buffer);
        self.scan_type.encode_into(buffer);
        buffer.put_u8(self.channel_page);
        self.phy_id.encode_into(buffer);
        self.unscanned_channels.encode_into(buffer);
        buffer.put_u8(self.result_list_count);
        buffer.extend(self.result_list.iter());
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x0c + self.result_list.len() as u8,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::AREQ,
                    subsystem: MTSubsystem::MAC,
                    id: 0x8c,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_ScanCnf_AREQ(self),
        }
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
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        let src_addr = Address::try_decode(Read::by_ref(cursor))?;
        let dst_addr = Address::try_decode(Read::by_ref(cursor))?;
        let device_pan_id = cursor.get_u16_le();
        let reason = CommEventReason::try_decode(Read::by_ref(cursor))?;
        let key_source = KeySource::try_decode(Read::by_ref(cursor))?;
        let security_level = SecurityLevel::try_decode(Read::by_ref(cursor))?;
        let key_id_mode = KeyIdMode::try_decode(Read::by_ref(cursor))?;
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

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.status.encode_into(buffer);
        self.src_addr.encode_into(buffer);
        self.dst_addr.encode_into(buffer);
        buffer.put_u16_le(self.device_pan_id);
        self.reason.encode_into(buffer);
        self.key_source.encode_into(buffer);
        self.security_level.encode_into(buffer);
        self.key_id_mode.encode_into(buffer);
        buffer.put_u8(self.key_index);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x21,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::AREQ,
                    subsystem: MTSubsystem::MAC,
                    id: 0x8d,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_CommStatusInd_AREQ(self),
        }
    }
}

#[derive(Debug)]
pub struct StartCnf {
    pub status: Status,
}

impl StartCnf {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(StartCnf { status })
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
                    cmd_type: CommandType::AREQ,
                    subsystem: MTSubsystem::MAC,
                    id: 0x8e,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_StartCnf_AREQ(self),
        }
    }
}

#[derive(Debug)]
pub struct WSAsyncCnf {
    pub status: Status,
}

impl WSAsyncCnf {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let status = Status::try_decode(Read::by_ref(cursor))?;
        Ok(WSAsyncCnf { status })
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
                    cmd_type: CommandType::AREQ,
                    subsystem: MTSubsystem::MAC,
                    id: 0x92,
                },
            },
            extended_header: None,
            payload: MTFramePayload::MAC_WSAsyncCnf_AREQ(self),
        }
    }
}
