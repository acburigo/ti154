pub mod mac;
pub mod rpc;
pub mod sys;
pub mod util;

use crate::error::Error;
use crate::frame::{MTExtendedHeader, MTHeader};
use crate::types::MTSubsystem;
use std::io::Cursor;

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

impl MTFramePayload {
    pub fn try_decode(
        header: &MTHeader,
        extended_header: &Option<MTExtendedHeader>,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<Self, Error> {
        match header.command.subsystem {
            MTSubsystem::MAC => mac::try_decode(header, extended_header, cursor),
            MTSubsystem::RPC => rpc::try_decode(header, extended_header, cursor),
            MTSubsystem::SYS => sys::try_decode(header, extended_header, cursor),
            MTSubsystem::UTIL => util::try_decode(header, extended_header, cursor),
        }
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        use MTFramePayload::*;
        match self {
            // MAC
            MAC_DataCnf_AREQ(x) => x.encode_into(buffer),
            MAC_DataInd_AREQ(x) => x.encode_into(buffer),
            MAC_PurgeCnf_AREQ(x) => x.encode_into(buffer),
            MAC_WSAsyncInd_AREQ(x) => x.encode_into(buffer),
            MAC_SyncLossInd_AREQ(x) => x.encode_into(buffer),
            MAC_AssociateInd_AREQ(x) => x.encode_into(buffer),
            MAC_AssociateCnf_AREQ(x) => x.encode_into(buffer),
            MAC_BeaconNotifyInd_AREQ(x) => x.encode_into(buffer),
            MAC_DisassociateInd_AREQ(x) => x.encode_into(buffer),
            MAC_DisassociateCnf_AREQ(x) => x.encode_into(buffer),
            MAC_OrphanInd_AREQ(x) => x.encode_into(buffer),
            MAC_PollCnf_AREQ(x) => x.encode_into(buffer),
            MAC_PollInd_AREQ(x) => x.encode_into(buffer),
            MAC_ScanCnf_AREQ(x) => x.encode_into(buffer),
            MAC_CommStatusInd_AREQ(x) => x.encode_into(buffer),
            MAC_StartCnf_AREQ(x) => x.encode_into(buffer),
            MAC_WSAsyncCnf_AREQ(x) => x.encode_into(buffer),

            MAC_Init_SREQ(x) => x.encode_into(buffer),
            MAC_DataReq_SREQ(x) => x.encode_into(buffer),
            MAC_PurgeReq_SREQ(x) => x.encode_into(buffer),
            MAC_AssociateReq_SREQ(x) => x.encode_into(buffer),
            MAC_AssociateRsp_SREQ(x) => x.encode_into(buffer),
            MAC_DisassociateReq_SREQ(x) => x.encode_into(buffer),
            MAC_GetReq_SREQ(x) => x.encode_into(buffer),
            MAC_SetReq_SREQ(x) => x.encode_into(buffer),
            MAC_SecurityGetReq_SREQ(x) => x.encode_into(buffer),
            MAC_SecuritySetReq_SREQ(x) => x.encode_into(buffer),
            MAC_UpdatePANIdReq_SREQ(x) => x.encode_into(buffer),
            MAC_AddDeviceReq_SREQ(x) => x.encode_into(buffer),
            MAC_DeleteDeviceReq_SREQ(x) => x.encode_into(buffer),
            MAC_DeleteAllDevicesReq_SREQ(x) => x.encode_into(buffer),
            MAC_DeleteKeyReq_SREQ(x) => x.encode_into(buffer),
            MAC_ReadKeyReq_SREQ(x) => x.encode_into(buffer),
            MAC_WriteKeyReq_SREQ(x) => x.encode_into(buffer),
            MAC_OrphanRsp_SREQ(x) => x.encode_into(buffer),
            MAC_PollReq_SREQ(x) => x.encode_into(buffer),
            MAC_ResetReq_SREQ(x) => x.encode_into(buffer),
            MAC_ScanReq_SREQ(x) => x.encode_into(buffer),
            MAC_StartReq_SREQ(x) => x.encode_into(buffer),
            MAC_SyncReq_SREQ(x) => x.encode_into(buffer),
            MAC_SetRxGainReq_SREQ(x) => x.encode_into(buffer),
            MAC_WSAsyncReq_SREQ(x) => x.encode_into(buffer),
            MAC_FHEnableReq_SREQ(x) => x.encode_into(buffer),
            MAC_FHStartReq_SREQ(x) => x.encode_into(buffer),
            MAC_FHGetReq_SREQ(x) => x.encode_into(buffer),
            MAC_FHSetReq_SREQ(x) => x.encode_into(buffer),

            MAC_Init_SRSP(x) => x.encode_into(buffer),
            MAC_DataReq_SRSP(x) => x.encode_into(buffer),
            MAC_PurgeReq_SRSP(x) => x.encode_into(buffer),
            MAC_AssociateReq_SRSP(x) => x.encode_into(buffer),
            MAC_AssociateRsp_SRSP(x) => x.encode_into(buffer),
            MAC_DisassociateReq_SRSP(x) => x.encode_into(buffer),
            MAC_GetReq_SRSP(x) => x.encode_into(buffer),
            MAC_SetReq_SRSP(x) => x.encode_into(buffer),
            MAC_SecurityGetReq_SRSP(x) => x.encode_into(buffer),
            MAC_SecuritySetReq_SRSP(x) => x.encode_into(buffer),
            MAC_UpdatePANIdReq_SRSP(x) => x.encode_into(buffer),
            MAC_AddDeviceReq_SRSP(x) => x.encode_into(buffer),
            MAC_DeleteDeviceReq_SRSP(x) => x.encode_into(buffer),
            MAC_DeleteAllDevicesReq_SRSP(x) => x.encode_into(buffer),
            MAC_DeleteKeyReq_SRSP(x) => x.encode_into(buffer),
            MAC_ReadKeyReq_SRSP(x) => x.encode_into(buffer),
            MAC_WriteKeyReq_SRSP(x) => x.encode_into(buffer),
            MAC_OrphanRsp_SRSP(x) => x.encode_into(buffer),
            MAC_PollReq_SRSP(x) => x.encode_into(buffer),
            MAC_ResetReq_SRSP(x) => x.encode_into(buffer),
            MAC_ScanReq_SRSP(x) => x.encode_into(buffer),
            MAC_StartReq_SRSP(x) => x.encode_into(buffer),
            MAC_SyncReq_SRSP(x) => x.encode_into(buffer),
            MAC_SetRxGainReq_SRSP(x) => x.encode_into(buffer),
            MAC_WSAsyncReq_SRSP(x) => x.encode_into(buffer),
            MAC_FHEnableReq_SRSP(x) => x.encode_into(buffer),
            MAC_FHStartReq_SRSP(x) => x.encode_into(buffer),
            MAC_FHGetReq_SRSP(x) => x.encode_into(buffer),
            MAC_FHSetReq_SRSP(x) => x.encode_into(buffer),

            // RPC
            RPC_MTCommandError(x) => x.encode_into(buffer),

            // SYS
            SYS_ResetReq_AREQ(x) => x.encode_into(buffer),
            SYS_ResetInd_AREQ(x) => x.encode_into(buffer),

            SYS_PingReq_SREQ(x) => x.encode_into(buffer),
            SYS_VersionReq_SREQ(x) => x.encode_into(buffer),
            SYS_NVCreateReq_SREQ(x) => x.encode_into(buffer),
            SYS_NVDeleteReq_SREQ(x) => x.encode_into(buffer),
            SYS_NVLengthReq_SREQ(x) => x.encode_into(buffer),
            SYS_NVReadReq_SREQ(x) => x.encode_into(buffer),
            SYS_NVWriteReq_SREQ(x) => x.encode_into(buffer),
            SYS_NVUpdateReq_SREQ(x) => x.encode_into(buffer),
            SYS_NVCompactReq_SREQ(x) => x.encode_into(buffer),

            SYS_PingReq_SRSP(x) => x.encode_into(buffer),
            SYS_VersionReq_SRSP(x) => x.encode_into(buffer),
            SYS_NVCreateReq_SRSP(x) => x.encode_into(buffer),
            SYS_NVDeleteReq_SRSP(x) => x.encode_into(buffer),
            SYS_NVLengthReq_SRSP(x) => x.encode_into(buffer),
            SYS_NVReadReq_SRSP(x) => x.encode_into(buffer),
            SYS_NVWriteReq_SRSP(x) => x.encode_into(buffer),
            SYS_NVUpdateReq_SRSP(x) => x.encode_into(buffer),
            SYS_NVCompactReq_SRSP(x) => x.encode_into(buffer),

            // UTIL
            UTIL_Loopback_AREQ(x) => x.encode_into(buffer),

            UTIL_CallbackSubCmd_SREQ(x) => x.encode_into(buffer),
            UTIL_GetExtAddr_SREQ(x) => x.encode_into(buffer),
            UTIL_Loopback_SREQ(x) => x.encode_into(buffer),
            UTIL_Random_SREQ(x) => x.encode_into(buffer),

            UTIL_CallbackSubCmd_SRSP(x) => x.encode_into(buffer),
            UTIL_GetExtAddr_SRSP(x) => x.encode_into(buffer),
            UTIL_Loopback_SRSP(x) => x.encode_into(buffer),
            UTIL_Random_SRSP(x) => x.encode_into(buffer),
        }
    }
}
