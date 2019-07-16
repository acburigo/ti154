use crate::error::Error;
use crate::subsystem::{mac, rpc, sys, util};
use crate::types::{CommandType, MTExtendedHeaderStatus, MTSubsystem};
use bytes::Buf;
use num_traits::FromPrimitive;
use std::convert::TryFrom;
use std::io::Cursor;

#[derive(Debug)]
pub struct MTFrame {
    pub header: MTHeader,
    pub extended_header: Option<MTExtendedHeader>,
    pub payload: MTFramePayload,
}

impl TryFrom<&mut Cursor<&[u8]>> for MTFrame {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let header = MTHeader::try_from(cursor.by_ref())?;
        let extended_header = if header.has_extension() {
            Some(MTExtendedHeader::try_from(cursor.by_ref())?)
        } else {
            None
        };
        let payload = MTFramePayload::try_from(cursor.by_ref())?;

        Ok(MTFrame {
            header,
            extended_header,
            payload,
        })
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum MTFramePayload {
    // MAC
    MAC_DataCnf_AREQ(mac::areq::DataCnf),
    MAC_DataInd_AREQ(mac::areq::DataInd),
    MAC_PurgeCnf_AREQ(mac::areq::PurgeCnf),
    MAC_WSAsyncInd_AREQ(mac::areq::WSAsyncInd),
    MAC_SyncLossInd_AREQ(mac::areq::SyncLossInd),
    MAC_AssociateInd_AREQ(mac::areq::AssociateInd),
    MAC_AssociateCnf_AREQ(mac::areq::AssociateCnf),
    MAC_BeaconNotifyInd_AREQ(mac::areq::BeaconNotifyInd),
    MAC_StandardBeaconFrame_AREQ(mac::areq::StandardBeaconFrame),
    MAC_EnhancedBeaconFrame_AREQ(mac::areq::EnhancedBeaconFrame),
    MAC_DisassociateInd_AREQ(mac::areq::DisassociateInd),
    MAC_DisassociateCnf_AREQ(mac::areq::DisassociateCnf),
    MAC_OrphanInd_AREQ(mac::areq::OrphanInd),
    MAC_PollCnf_AREQ(mac::areq::PollCnf),
    MAC_PollInd_AREQ(mac::areq::PollInd),
    MAC_ScanCnf_AREQ(mac::areq::ScanCnf),
    MAC_CommStatusInd_AREQ(mac::areq::CommStatusInd),
    MAC_StartCnf_AREQ(mac::areq::StartCnf),
    MAC_WSAsyncCnf_AREQ(mac::areq::WSAsyncCnf),

    MAC_Init_SREQ(mac::sreq::Init),
    MAC_DataReq_SREQ(mac::sreq::DataReq),
    MAC_PurgeReq_SREQ(mac::sreq::PurgeReq),
    MAC_AssociateReq_SREQ(mac::sreq::AssociateReq),
    MAC_AssociateRsp_SREQ(mac::sreq::AssociateRsp),
    MAC_DisassociateReq_SREQ(mac::sreq::DisassociateReq),
    MAC_GetReq_SREQ(mac::sreq::GetReq),
    MAC_SetReq_SREQ(mac::sreq::SetReq),
    MAC_SecurityGetReq_SREQ(mac::sreq::SecurityGetReq),
    MAC_SecuritySetReq_SREQ(mac::sreq::SecuritySetReq),
    MAC_UpdatePANIdReq_SREQ(mac::sreq::UpdatePANIdReq),
    MAC_AddDeviceReq_SREQ(mac::sreq::AddDeviceReq),
    MAC_DeleteDeviceReq_SREQ(mac::sreq::DeleteDeviceReq),
    MAC_DeleteAllDevicesReq_SREQ(mac::sreq::DeleteAllDevicesReq),
    MAC_DeleteKeyReq_SREQ(mac::sreq::DeleteKeyReq),
    MAC_ReadKeyReq_SREQ(mac::sreq::ReadKeyReq),
    MAC_WriteKeyReq_SREQ(mac::sreq::WriteKeyReq),
    MAC_OrphanRsp_SREQ(mac::sreq::OrphanRsp),
    MAC_PollReq_SREQ(mac::sreq::PollReq),
    MAC_ResetReq_SREQ(mac::sreq::ResetReq),
    MAC_ScanReq_SREQ(mac::sreq::ScanReq),
    MAC_StartReq_SREQ(mac::sreq::StartReq),
    MAC_SyncReq_SREQ(mac::sreq::SyncReq),
    MAC_SetRxGainReq_SREQ(mac::sreq::SetRxGainReq),
    MAC_WSAsyncReq_SREQ(mac::sreq::WSAsyncReq),
    MAC_FHEnableReq_SREQ(mac::sreq::FHEnableReq),
    MAC_FHStartReq_SREQ(mac::sreq::FHStartReq),
    MAC_FHGetReq_SREQ(mac::sreq::FHGetReq),
    MAC_FHSetReq_SREQ(mac::sreq::FHSetReq),

    MAC_Init_SRSP(mac::srsp::Init),
    MAC_DataReq_SRSP(mac::srsp::DataReq),
    MAC_PurgeReq_SRSP(mac::srsp::PurgeReq),
    MAC_AssociateReq_SRSP(mac::srsp::AssociateReq),
    MAC_AssociateRsp_SRSP(mac::srsp::AssociateRsp),
    MAC_DisassociateReq_SRSP(mac::srsp::DisassociateReq),
    MAC_GetReq_SRSP(mac::srsp::GetReq),
    MAC_SetReq_SRSP(mac::srsp::SetReq),
    MAC_SecurityGetReq_SRSP(mac::srsp::SecurityGetReq),
    MAC_SecuritySetReq_SRSP(mac::srsp::SecuritySetReq),
    MAC_UpdatePANIdReq_SRSP(mac::srsp::UpdatePANIdReq),
    MAC_AddDeviceReq_SRSP(mac::srsp::AddDeviceReq),
    MAC_DeleteDeviceReq_SRSP(mac::srsp::DeleteDeviceReq),
    MAC_DeleteAllDevicesReq_SRSP(mac::srsp::DeleteAllDevicesReq),
    MAC_DeleteKeyReq_SRSP(mac::srsp::DeleteKeyReq),
    MAC_ReadKeyReq_SRSP(mac::srsp::ReadKeyReq),
    MAC_WriteKeyReq_SRSP(mac::srsp::WriteKeyReq),
    MAC_OrphanRsp_SRSP(mac::srsp::OrphanRsp),
    MAC_PollReq_SRSP(mac::srsp::PollReq),
    MAC_ResetReq_SRSP(mac::srsp::ResetReq),
    MAC_ScanReq_SRSP(mac::srsp::ScanReq),
    MAC_StartReq_SRSP(mac::srsp::StartReq),
    MAC_SyncReq_SRSP(mac::srsp::SyncReq),
    MAC_SetRxGainReq_SRSP(mac::srsp::SetRxGainReq),
    MAC_WSAsyncReq_SRSP(mac::srsp::WSAsyncReq),
    MAC_FHEnableReq_SRSP(mac::srsp::FHEnableReq),
    MAC_FHStartReq_SRSP(mac::srsp::FHStartReq),
    MAC_FHGetReq_SRSP(mac::srsp::FHGetReq),
    MAC_FHSetReq_SRSP(mac::srsp::FHSetReq),

    // RPC
    RPC_MTCommandError(rpc::MTCommandError),

    // SYS
    SYS_ResetReq_AREQ(sys::areq::ResetReq),
    SYS_ResetInd_AREQ(sys::areq::ResetInd),

    SYS_PingReq_SREQ(sys::sreq::PingReq),
    SYS_VersionReq_SREQ(sys::sreq::VersionReq),
    SYS_NVCreateReq_SREQ(sys::sreq::NVCreateReq),
    SYS_NVDeleteReq_SREQ(sys::sreq::NVDeleteReq),
    SYS_NVLengthReq_SREQ(sys::sreq::NVLengthReq),
    SYS_NVReadReq_SREQ(sys::sreq::NVReadReq),
    SYS_NVWriteReq_SREQ(sys::sreq::NVWriteReq),
    SYS_NVUpdateReq_SREQ(sys::sreq::NVUpdateReq),
    SYS_NVCompactReq_SREQ(sys::sreq::NVCompactReq),

    SYS_PingReq_SRSP(sys::srsp::PingReq),
    SYS_VersionReq_SRSP(sys::srsp::VersionReq),
    SYS_NVCreateReq_SRSP(sys::srsp::NVCreateReq),
    SYS_NVDeleteReq_SRSP(sys::srsp::NVDeleteReq),
    SYS_NVLengthReq_SRSP(sys::srsp::NVLengthReq),
    SYS_NVReadReq_SRSP(sys::srsp::NVReadReq),
    SYS_NVWriteReq_SRSP(sys::srsp::NVWriteReq),
    SYS_NVUpdateReq_SRSP(sys::srsp::NVUpdateReq),
    SYS_NVCompactReq_SRSP(sys::srsp::NVCompactReq),

    // UTIL
    UTIL_Loopback_AREQ(util::areq::Loopback),

    UTIL_CallbackSubCmd_SREQ(util::sreq::CallbackSubCmd),
    UTIL_GetExtAddr_SREQ(util::sreq::GetExtAddr),
    UTIL_Loopback_SREQ(util::sreq::Loopback),
    UTIL_Random_SREQ(util::sreq::Random),

    UTIL_CallbackSubCmd_SRSP(util::srsp::CallbackSubCmd),
    UTIL_GetExtAddr_SRSP(util::srsp::GetExtAddr),
    UTIL_Loopback_SRSP(util::srsp::Loopback),
    UTIL_Random_SRSP(util::srsp::Random),
}

impl TryFrom<&mut Cursor<&[u8]>> for MTFramePayload {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        Err(Error::NotImplemented)
    }
}

#[derive(Debug)]
pub struct MTHeader {
    pub length: u8,
    pub command: CommandCode,
}

impl MTHeader {
    pub fn size() -> usize {
        return 3;
    }

    pub fn has_extension(&self) -> bool {
        self.command.is_extended
    }
}

impl TryFrom<&mut Cursor<&[u8]>> for MTHeader {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let length = cursor.get_u8();
        let command = CommandCode::try_from(cursor)?;
        Ok(MTHeader { length, command })
    }
}

#[derive(Debug)]
pub struct CommandCode {
    pub is_extended: bool,
    pub cmd_type: CommandType,
    pub subsystem: MTSubsystem,
    pub id: u8,
}

impl TryFrom<&mut Cursor<&[u8]>> for CommandCode {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let type_and_subsystem = cursor.get_u8();
        let id = cursor.get_u8();

        let is_extended = (type_and_subsystem & 0x80) != 0;

        let cmd_type = 0x03 & (type_and_subsystem >> 5);
        let cmd_type =
            FromPrimitive::from_u8(cmd_type).ok_or(Error::InvalidCommandType(cmd_type))?;

        let subsystem = type_and_subsystem & 0x1F;
        let subsystem =
            FromPrimitive::from_u8(subsystem).ok_or(Error::InvalidSubsystem(subsystem))?;

        Ok(CommandCode {
            is_extended,
            cmd_type,
            subsystem,
            id,
        })
    }
}

#[derive(Debug)]
pub enum MTExtendedHeader {
    V1 {
        stack_id: u8,
    },
    V2 {
        stack_id: u8,
        block: u8,
        packet_length: u16,
    },
    V3 {
        stack_id: u8,
        block: u8,
        status: MTExtendedHeaderStatus,
    },
    V4 {
        stack_id: u8,
        block: u8,
        status: MTExtendedHeaderStatus,
    },
}

impl TryFrom<&mut Cursor<&[u8]>> for MTExtendedHeader {
    type Error = Error;
    fn try_from(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let version_and_stack_id = cursor.get_u8();
        let version = version_and_stack_id & 0xf8;
        let stack_id = version_and_stack_id & 0x07;

        if version == 1 {
            return Ok(MTExtendedHeader::V1 { stack_id });
        }

        let block = cursor.get_u8();

        if version == 2 {
            let packet_length = cursor.get_u16_le();
            return Ok(MTExtendedHeader::V2 {
                stack_id,
                block,
                packet_length,
            });
        }

        if version == 3 || version == 4 {
            let status = cursor.get_u8();
            let status =
                FromPrimitive::from_u8(status).ok_or(Error::InvalidExtendedHeaderStatus(status))?;

            if version == 3 {
                return Ok(MTExtendedHeader::V3 {
                    stack_id,
                    block,
                    status,
                });
            } else {
                return Ok(MTExtendedHeader::V4 {
                    stack_id,
                    block,
                    status,
                });
            }
        }

        Err(Error::NotImplemented)
    }
}
