use crate::error::Error;
use crate::frame::{CommandCode, MTFrame, MTHeader};
use crate::types::*;
use bytes::{Buf, BufMut};
use std::io::Cursor;
use std::io::Read;

#[derive(Debug)]
pub struct Init {}

impl Init {
    pub fn try_decode(_: &[u8]) -> Result<Self, Error> {
        Ok(Init {})
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, _: &mut Vec<u8>) {}

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x00,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::Init as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
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
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let dest_address = Address::try_decode(&mut cursor)?;
        let dest_pan_id = cursor.get_u16_le();
        let src_address_mode = AddressMode::try_decode(&mut cursor)?;
        let handle = cursor.get_u8();
        let tx_option = TxOption::try_decode(&mut cursor)?;
        let channel = cursor.get_u8();
        let power = cursor.get_u8();
        let key_source = KeySource::try_decode(&mut cursor)?;
        let security_level = SecurityLevel::try_decode(&mut cursor)?;
        let key_id_mode = KeyIdMode::try_decode(&mut cursor)?;
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

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.dest_address.encode_into(buffer);
        buffer.put_u16_le(self.dest_pan_id);
        self.src_address_mode.encode_into(buffer);
        buffer.put_u8(self.handle);
        self.tx_option.encode_into(buffer);
        buffer.put_u8(self.channel);
        buffer.put_u8(self.power);
        self.key_source.encode_into(buffer);
        self.security_level.encode_into(buffer);
        self.key_id_mode.encode_into(buffer);
        buffer.put_u8(self.key_index);
        buffer.put_u32_le(self.include_fh_ies);
        buffer.put_u16_le(self.data_length);
        buffer.put_u16_le(self.ie_length);
        buffer.extend(self.data_payload.iter());
        buffer.extend(self.ie_payload.iter());
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: (0x23 + self.data_payload.len() + self.ie_payload.len()) as u8,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::DataReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug)]
pub struct PurgeReq {
    pub handle: u8,
}

impl PurgeReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let handle = cursor.get_u8();
        Ok(PurgeReq { handle })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(self.handle);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x01,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::PurgeReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
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
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let logical_channel = cursor.get_u8();
        let channel_page = cursor.get_u8();
        let phy_id = cursor.get_u8();
        let coord_address = Address::try_decode(&mut cursor)?;
        let coord_pan_id = cursor.get_u16_le();
        let capability_info = cursor.get_u8();
        let key_source = KeySource::try_decode(&mut cursor)?;
        let security_level = SecurityLevel::try_decode(&mut cursor)?;
        let key_id_mode = KeyIdMode::try_decode(&mut cursor)?;
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

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(self.logical_channel);
        buffer.put_u8(self.channel_page);
        buffer.put_u8(self.phy_id);
        self.coord_address.encode_into(buffer);
        buffer.put_u16_le(self.coord_pan_id);
        buffer.put_u8(self.capability_info);
        self.key_source.encode_into(buffer);
        self.security_level.encode_into(buffer);
        self.key_id_mode.encode_into(buffer);
        buffer.put_u8(self.key_index);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x1a,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::AssociateReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
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
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let extended_address = ExtendedAddress::try_decode(&mut cursor)?;
        let assoc_short_address = ShortAddress::try_decode(&mut cursor)?;
        let assoc_status = AssociationStatus::try_decode(&mut cursor)?;
        let key_source = KeySource::try_decode(&mut cursor)?;
        let security_level = SecurityLevel::try_decode(&mut cursor)?;
        let key_id_mode = KeyIdMode::try_decode(&mut cursor)?;
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

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.extended_address.encode_into(buffer);
        self.assoc_short_address.encode_into(buffer);
        self.assoc_status.encode_into(buffer);
        self.key_source.encode_into(buffer);
        self.security_level.encode_into(buffer);
        self.key_id_mode.encode_into(buffer);
        buffer.put_u8(self.key_index);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x16,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::AssociateRsp as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug)]
pub struct DisassociateReq {
    pub device_address: Address,
    pub device_pan_id: u16,
    pub disassociate_reason: DisassociateReason,
    pub tx_indirect: bool,
    pub key_source: KeySource,
    pub security_level: SecurityLevel,
    pub key_id_mode: KeyIdMode,
    pub key_index: u8,
}

impl DisassociateReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let device_address = Address::try_decode(&mut cursor)?;
        let device_pan_id = cursor.get_u16_le();
        let disassociate_reason = DisassociateReason::try_decode(&mut cursor)?;
        let tx_indirect = cursor.get_u8() == 0;
        let key_source = KeySource::try_decode(&mut cursor)?;
        let security_level = SecurityLevel::try_decode(&mut cursor)?;
        let key_id_mode = KeyIdMode::try_decode(&mut cursor)?;
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

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.device_address.encode_into(buffer);
        buffer.put_u16_le(self.device_pan_id);
        self.disassociate_reason.encode_into(buffer);
        buffer.put_u8(if self.tx_indirect { 1 } else { 0 });
        self.key_source.encode_into(buffer);
        self.security_level.encode_into(buffer);
        self.key_id_mode.encode_into(buffer);
        buffer.put_u8(self.key_index);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x18,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::DisassociateReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug)]
pub struct GetReq {
    pub attribute_id: MACPIBAttributeId,
}

impl GetReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let attribute_id = MACPIBAttributeId::try_decode(&mut cursor)?;
        Ok(GetReq { attribute_id })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.attribute_id.encode_into(buffer);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x01,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::GetReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug)]
pub struct SetReq {
    pub attribute_id: MACPIBAttributeId,
    pub attribute_value: [u8; 16],
}

impl SetReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let attribute_id = MACPIBAttributeId::try_decode(&mut cursor)?;

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

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.attribute_id.encode_into(buffer);
        buffer.extend(self.attribute_value.iter().rev());
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x11,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::SetReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug)]
pub struct SecurityGetReq {
    pub attribute_id: SecurityPIBAttributeId,
    pub index1: u16,
    pub index2: u16,
}

impl SecurityGetReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let attribute_id = SecurityPIBAttributeId::try_decode(&mut cursor)?;
        let index1 = cursor.get_u16_le();
        let index2 = cursor.get_u16_le();

        Ok(SecurityGetReq {
            attribute_id,
            index1,
            index2,
        })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.attribute_id.encode_into(buffer);
        buffer.put_u16_le(self.index1);
        buffer.put_u16_le(self.index2);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x05,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::SecurityGetReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug)]
pub struct SecuritySetReq {
    pub attribute_id: SecurityPIBAttributeId,
    pub index1: u16,
    pub index2: u16,
    pub attribute_value: Vec<u8>,
}

impl SecuritySetReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let attribute_id = SecurityPIBAttributeId::try_decode(&mut cursor)?;
        let index1 = cursor.get_u16_le();
        let index2 = cursor.get_u16_le();

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

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.attribute_id.encode_into(buffer);
        buffer.put_u16_le(self.index1);
        buffer.put_u16_le(self.index2);
        buffer.extend(self.attribute_value.iter());
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x05 + self.attribute_value.len() as u8,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::SecuritySetReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug)]
pub struct UpdatePANIdReq {
    pub pan_id: u16,
}

impl UpdatePANIdReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let pan_id = cursor.get_u16_le();
        Ok(UpdatePANIdReq { pan_id })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u16_le(self.pan_id);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x02,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::UpdatePANIdReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
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
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let pan_id = cursor.get_u16_le();
        let short_addr = ShortAddress::try_decode(&mut cursor)?;
        let ext_addr = ExtendedAddress::try_decode(&mut cursor)?;
        let frame_counter = cursor.get_u32_le();
        let exempt = cursor.get_u8() != 0;
        let unique = cursor.get_u8() != 0;
        let duplicate = cursor.get_u8() != 0;
        let data_size = cursor.get_u8();

        let mut lookup_data: [u8; 9] = Default::default();
        cursor
            .read_exact(&mut lookup_data)
            .map_err(|_| Error::NotEnoughBytes)?;

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

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u16_le(self.pan_id);
        self.short_addr.encode_into(buffer);
        self.ext_addr.encode_into(buffer);
        buffer.put_u32_le(self.frame_counter);
        buffer.put_u8(if self.exempt { 1 } else { 0 });
        buffer.put_u8(if self.unique { 1 } else { 0 });
        buffer.put_u8(if self.duplicate { 1 } else { 0 });
        buffer.put_u8(self.data_size);
        buffer.extend(self.lookup_data.iter());
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x1d,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::AddDeviceReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug)]
pub struct DeleteDeviceReq {
    pub ext_addr: ExtendedAddress,
}

impl DeleteDeviceReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let ext_addr = ExtendedAddress::try_decode(&mut cursor)?;
        Ok(DeleteDeviceReq { ext_addr })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.ext_addr.encode_into(buffer);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x08,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::DeleteDeviceReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug)]
pub struct DeleteAllDevicesReq {}

impl DeleteAllDevicesReq {
    pub fn try_decode(_: &[u8]) -> Result<Self, Error> {
        Ok(DeleteAllDevicesReq {})
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, _: &mut Vec<u8>) {}

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x00,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::DeleteAllDevicesReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug)]
pub struct DeleteKeyReq {
    pub index: u8,
}

impl DeleteKeyReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let index = cursor.get_u8();
        Ok(DeleteKeyReq { index })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(self.index);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x01,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::DeleteKeyReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug)]
pub struct ReadKeyReq {
    pub index: u8,
}

impl ReadKeyReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let index = cursor.get_u8();
        Ok(ReadKeyReq { index })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(self.index);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x01,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::ReadKeyReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
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
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let new = cursor.get_u8() != 0;
        let index = cursor.get_u16_le();

        let mut key: [u8; 16] = Default::default();
        cursor
            .read_exact(&mut key)
            .map_err(|_| Error::NotEnoughBytes)?;

        let frame_counter = cursor.get_u32_le();
        let data_size = cursor.get_u8();

        let mut lookup_data: [u8; 9] = Default::default();
        cursor
            .read_exact(&mut lookup_data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(WriteKeyReq {
            new,
            index,
            key,
            frame_counter,
            data_size,
            lookup_data,
        })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(if self.new { 1 } else { 0 });
        buffer.put_u16_le(self.index);
        buffer.extend(self.key.iter());
        buffer.put_u32_le(self.frame_counter);
        buffer.put_u8(self.data_size);
        buffer.extend(self.lookup_data.iter());
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x21,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::WriteKeyReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
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
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let extended_address = ExtendedAddress::try_decode(&mut cursor)?;
        let assoc_short_address = ShortAddress::try_decode(&mut cursor)?;
        let associated_member = cursor.get_u8() != 0;
        let key_source = KeySource::try_decode(&mut cursor)?;
        let security_level = SecurityLevel::try_decode(&mut cursor)?;
        let key_id_mode = KeyIdMode::try_decode(&mut cursor)?;
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

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.extended_address.encode_into(buffer);
        self.assoc_short_address.encode_into(buffer);
        buffer.put_u8(if self.associated_member { 1 } else { 0 });
        self.key_source.encode_into(buffer);
        self.security_level.encode_into(buffer);
        self.key_id_mode.encode_into(buffer);
        buffer.put_u8(self.key_index);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x16,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::OrphanRsp as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
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
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let coord_address = Address::try_decode(&mut cursor)?;
        let coord_pan_id = cursor.get_u16_le();
        let key_source = KeySource::try_decode(&mut cursor)?;
        let security_level = SecurityLevel::try_decode(&mut cursor)?;
        let key_id_mode = KeyIdMode::try_decode(&mut cursor)?;
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

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.coord_address.encode_into(buffer);
        buffer.put_u16_le(self.coord_pan_id);
        self.key_source.encode_into(buffer);
        self.security_level.encode_into(buffer);
        self.key_id_mode.encode_into(buffer);
        buffer.put_u8(self.key_index);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x16,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::PollReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug)]
pub struct ResetReq {
    pub set_default: bool,
}

impl ResetReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let set_default = cursor.get_u8() != 0;
        Ok(ResetReq { set_default })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(if self.set_default { 1 } else { 0 });
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x01,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::ResetReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
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
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let scan_type = ScanType::try_decode(&mut cursor)?;
        let scan_duration = cursor.get_u8();
        let channel_page = cursor.get_u8();
        let phy_id = PhyId::try_decode(&mut cursor)?;
        let max_results = cursor.get_u8();
        let permit_join = PermitJoin::try_decode(&mut cursor)?;
        let link_quality = cursor.get_u8();
        let rsp_filter = cursor.get_u8();
        let mpm_scan = MPMScan::try_decode(&mut cursor)?;
        let mpm_type = MPMType::try_decode(&mut cursor)?;
        let mpm_duration = cursor.get_u16_le();
        let key_source = KeySource::try_decode(&mut cursor)?;
        let security_level = SecurityLevel::try_decode(&mut cursor)?;
        let key_id_mode = KeyIdMode::try_decode(&mut cursor)?;
        let key_index = cursor.get_u8();
        let channels = ChannelsBitMap::try_decode(&mut cursor)?;
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

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.scan_type.encode_into(buffer);
        buffer.put_u8(self.scan_duration);
        buffer.put_u8(self.channel_page);
        self.phy_id.encode_into(buffer);
        buffer.put_u8(self.max_results);
        self.permit_join.encode_into(buffer);
        buffer.put_u8(self.link_quality);
        buffer.put_u8(self.rsp_filter);
        self.mpm_scan.encode_into(buffer);
        self.mpm_type.encode_into(buffer);
        buffer.put_u16_le(self.mpm_duration);
        self.key_source.encode_into(buffer);
        self.security_level.encode_into(buffer);
        self.key_id_mode.encode_into(buffer);
        buffer.put_u8(self.key_index);
        self.channels.encode_into(buffer);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x1b,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::ScanReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
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
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let start_time = cursor.get_u32_le();
        let pan_id = cursor.get_u16_le();
        let logical_channel = cursor.get_u8();
        let channel_page = cursor.get_u8();
        let phy_id = PhyId::try_decode(&mut cursor)?;
        let beacon_order = cursor.get_u8();
        let super_frame_order = cursor.get_u8();
        let pan_coordinator = cursor.get_u8() != 0;
        let battery_life_ext = cursor.get_u8() != 0;
        let coord_realignment = cursor.get_u8() != 0;
        let realign_key_source = KeySource::try_decode(&mut cursor)?;
        let realign_security_level = SecurityLevel::try_decode(&mut cursor)?;
        let realign_key_id_mode = KeyIdMode::try_decode(&mut cursor)?;
        let realign_key_index = cursor.get_u8();
        let beacon_key_source = KeySource::try_decode(&mut cursor)?;
        let beacon_security_level = SecurityLevel::try_decode(&mut cursor)?;
        let beacon_key_id_mode = KeyIdMode::try_decode(&mut cursor)?;
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

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u32_le(self.start_time);
        buffer.put_u16_le(self.pan_id);
        buffer.put_u8(self.logical_channel);
        buffer.put_u8(self.channel_page);
        self.phy_id.encode_into(buffer);
        buffer.put_u8(self.beacon_order);
        buffer.put_u8(self.super_frame_order);
        buffer.put_u8(if self.pan_coordinator { 1 } else { 0 });
        buffer.put_u8(if self.battery_life_ext { 1 } else { 0 });
        buffer.put_u8(if self.coord_realignment { 1 } else { 0 });
        self.realign_key_source.encode_into(buffer);
        self.realign_security_level.encode_into(buffer);
        self.realign_key_id_mode.encode_into(buffer);
        buffer.put_u8(self.realign_key_index);
        self.beacon_key_source.encode_into(buffer);
        self.beacon_security_level.encode_into(buffer);
        self.beacon_key_id_mode.encode_into(buffer);
        buffer.put_u8(self.beacon_key_index);
        buffer.put_u8(if self.start_fh { 1 } else { 0 });
        buffer.put_u8(self.enh_beacon_order);
        buffer.put_u8(self.ofs_time_slot);
        buffer.put_u16_le(self.non_beacon_order);
        buffer.put_u8(self.num_ies);
        buffer.extend(self.ie_id_list.iter());
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x2a + self.ie_id_list.len() as u8,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::StartReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
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
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let logical_channel = cursor.get_u8();
        let channel_page = cursor.get_u8();
        let track_beacon = cursor.get_u8() != 0;
        let phy_id = PhyId::try_decode(&mut cursor)?;
        Ok(SyncReq {
            logical_channel,
            channel_page,
            track_beacon,
            phy_id,
        })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(self.logical_channel);
        buffer.put_u8(self.channel_page);
        buffer.put_u8(if self.track_beacon { 1 } else { 0 });
        self.phy_id.encode_into(buffer);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x04,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::SyncReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug)]
pub struct SetRxGainReq {
    pub mode: bool,
}

impl SetRxGainReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let mode = cursor.get_u8() != 0;
        Ok(SetRxGainReq { mode })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(if self.mode { 1 } else { 0 });
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x01,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::SetRxGainReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
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
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let operation = WiSUNAsyncOperation::try_decode(&mut cursor)?;
        let frame_type = WiSUNAsyncFrameType::try_decode(&mut cursor)?;
        let key_source = KeySource::try_decode(&mut cursor)?;
        let security_level = SecurityLevel::try_decode(&mut cursor)?;
        let key_id_mode = KeyIdMode::try_decode(&mut cursor)?;
        let key_index = cursor.get_u8();
        let channels = ChannelsBitMap::try_decode(&mut cursor)?;
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

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.operation.encode_into(buffer);
        self.frame_type.encode_into(buffer);
        self.key_source.encode_into(buffer);
        self.security_level.encode_into(buffer);
        self.key_id_mode.encode_into(buffer);
        buffer.put_u8(self.key_index);
        self.channels.encode_into(buffer);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x26,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::WSAsyncReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug)]
pub struct FHEnableReq {}

impl FHEnableReq {
    pub fn try_decode(_: &[u8]) -> Result<Self, Error> {
        Ok(FHEnableReq {})
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, _: &mut Vec<u8>) {}

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x00,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::FHEnableReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug)]
pub struct FHStartReq {}

impl FHStartReq {
    pub fn try_decode(_: &[u8]) -> Result<Self, Error> {
        Ok(FHStartReq {})
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, _: &mut Vec<u8>) {}

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x00,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::FHStartReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug)]
pub struct FHGetReq {
    pub attribute_id: FHPIBAttributeId,
}

impl FHGetReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let attribute_id = FHPIBAttributeId::try_decode(&mut cursor)?;
        Ok(FHGetReq { attribute_id })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.attribute_id.encode_into(buffer);
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x02,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::FHGetReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}

#[derive(Debug)]
pub struct FHSetReq {
    pub attribute_id: FHPIBAttributeId,
    pub data: Vec<u8>,
}

impl FHSetReq {
    pub fn try_decode(buffer: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buffer);
        let attribute_id = FHPIBAttributeId::try_decode(&mut cursor)?;

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| Error::NotEnoughBytes)?;

        Ok(FHSetReq { attribute_id, data })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_into(&mut buffer);
        buffer
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        self.attribute_id.encode_into(buffer);
        buffer.extend(self.data.iter());
    }

    pub fn into_mt_frame(self) -> MTFrame {
        MTFrame {
            header: MTHeader {
                length: 0x02 + self.data.len() as u8,
                command: CommandCode {
                    is_extended: false,
                    cmd_type: CommandType::SREQ,
                    subsystem: MTSubsystem::MAC,
                    id: MACCommandId::FHSetReq as u8,
                },
            },
            extended_header: None,
            payload: self.encode(),
        }
    }
}
