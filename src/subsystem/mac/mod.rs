pub mod areq;
pub mod sreq;
pub mod srsp;

use crate::error::Error;
use crate::frame::{MTExtendedHeader, MTHeader};
use crate::subsystem::MTFramePayload;
use crate::types::CommandType;
use std::io::Cursor;

pub fn try_decode(
    header: &MTHeader,
    _: &Option<MTExtendedHeader>,
    cursor: &mut Cursor<&[u8]>,
) -> Result<MTFramePayload, Error> {
    use MTFramePayload::*;

    match header.command.cmd_type {
        CommandType::POLL => Err(Error::NotImplemented),
        CommandType::SREQ => match header.command.id {
            0x02 => sreq::Init::try_decode(cursor).map(|x| MAC_Init_SREQ(x)),
            0x05 => sreq::DataReq::try_decode(cursor).map(|x| MAC_DataReq_SREQ(x)),
            0x0e => sreq::PurgeReq::try_decode(cursor).map(|x| MAC_PurgeReq_SREQ(x)),
            0x06 => sreq::AssociateReq::try_decode(cursor).map(|x| MAC_AssociateReq_SREQ(x)),
            0x50 => sreq::AssociateRsp::try_decode(cursor).map(|x| MAC_AssociateRsp_SREQ(x)),
            0x07 => sreq::DisassociateReq::try_decode(cursor).map(|x| MAC_DisassociateReq_SREQ(x)),
            0x08 => sreq::GetReq::try_decode(cursor).map(|x| MAC_GetReq_SREQ(x)),
            0x09 => sreq::SetReq::try_decode(cursor).map(|x| MAC_SetReq_SREQ(x)),
            0x30 => sreq::SecurityGetReq::try_decode(cursor).map(|x| MAC_SecurityGetReq_SREQ(x)),
            0x31 => sreq::SecuritySetReq::try_decode(cursor).map(|x| MAC_SecuritySetReq_SREQ(x)),
            0x32 => sreq::UpdatePANIdReq::try_decode(cursor).map(|x| MAC_UpdatePANIdReq_SREQ(x)),
            0x33 => sreq::AddDeviceReq::try_decode(cursor).map(|x| MAC_AddDeviceReq_SREQ(x)),
            0x34 => sreq::DeleteDeviceReq::try_decode(cursor).map(|x| MAC_DeleteDeviceReq_SREQ(x)),
            0x35 => sreq::DeleteAllDevicesReq::try_decode(cursor)
                .map(|x| MAC_DeleteAllDevicesReq_SREQ(x)),
            0x36 => sreq::DeleteKeyReq::try_decode(cursor).map(|x| MAC_DeleteKeyReq_SREQ(x)),
            0x37 => sreq::ReadKeyReq::try_decode(cursor).map(|x| MAC_ReadKeyReq_SREQ(x)),
            0x38 => sreq::WriteKeyReq::try_decode(cursor).map(|x| MAC_WriteKeyReq_SREQ(x)),
            0x51 => sreq::OrphanRsp::try_decode(cursor).map(|x| MAC_OrphanRsp_SREQ(x)),
            0x0d => sreq::PollReq::try_decode(cursor).map(|x| MAC_PollReq_SREQ(x)),
            0x01 => sreq::ResetReq::try_decode(cursor).map(|x| MAC_ResetReq_SREQ(x)),
            0x0c => sreq::ScanReq::try_decode(cursor).map(|x| MAC_ScanReq_SREQ(x)),
            0x03 => sreq::StartReq::try_decode(cursor).map(|x| MAC_StartReq_SREQ(x)),
            0x04 => sreq::SyncReq::try_decode(cursor).map(|x| MAC_SyncReq_SREQ(x)),
            0x0f => sreq::SetRxGainReq::try_decode(cursor).map(|x| MAC_SetRxGainReq_SREQ(x)),
            0x44 => sreq::WSAsyncReq::try_decode(cursor).map(|x| MAC_WSAsyncReq_SREQ(x)),
            0x40 => sreq::FHEnableReq::try_decode(cursor).map(|x| MAC_FHEnableReq_SREQ(x)),
            0x41 => sreq::FHStartReq::try_decode(cursor).map(|x| MAC_FHStartReq_SREQ(x)),
            0x42 => sreq::FHGetReq::try_decode(cursor).map(|x| MAC_FHGetReq_SREQ(x)),
            0x43 => sreq::FHSetReq::try_decode(cursor).map(|x| MAC_FHSetReq_SREQ(x)),
            _ => Err(Error::NotImplemented),
        },
        CommandType::AREQ => match header.command.id {
            0x84 => areq::DataCnf::try_decode(cursor).map(|x| MAC_DataCnf_AREQ(x)),
            0x85 => areq::DataInd::try_decode(cursor).map(|x| MAC_DataInd_AREQ(x)),
            0x90 => areq::PurgeCnf::try_decode(cursor).map(|x| MAC_PurgeCnf_AREQ(x)),
            0x93 => areq::WSAsyncInd::try_decode(cursor).map(|x| MAC_WSAsyncInd_AREQ(x)),
            0x80 => areq::SyncLossInd::try_decode(cursor).map(|x| MAC_SyncLossInd_AREQ(x)),
            0x81 => areq::AssociateInd::try_decode(cursor).map(|x| MAC_AssociateInd_AREQ(x)),
            0x82 => areq::AssociateCnf::try_decode(cursor).map(|x| MAC_AssociateCnf_AREQ(x)),
            0x83 => areq::BeaconNotifyInd::try_decode(cursor).map(|x| MAC_BeaconNotifyInd_AREQ(x)),
            0x86 => areq::DisassociateInd::try_decode(cursor).map(|x| MAC_DisassociateInd_AREQ(x)),
            0x87 => areq::DisassociateCnf::try_decode(cursor).map(|x| MAC_DisassociateCnf_AREQ(x)),
            0x8a => areq::OrphanInd::try_decode(cursor).map(|x| MAC_OrphanInd_AREQ(x)),
            0x8b => areq::PollCnf::try_decode(cursor).map(|x| MAC_PollCnf_AREQ(x)),
            0x91 => areq::PollInd::try_decode(cursor).map(|x| MAC_PollInd_AREQ(x)),
            0x8c => areq::ScanCnf::try_decode(cursor).map(|x| MAC_ScanCnf_AREQ(x)),
            0x8d => areq::CommStatusInd::try_decode(cursor).map(|x| MAC_CommStatusInd_AREQ(x)),
            0x8e => areq::StartCnf::try_decode(cursor).map(|x| MAC_StartCnf_AREQ(x)),
            0x92 => areq::WSAsyncCnf::try_decode(cursor).map(|x| MAC_WSAsyncCnf_AREQ(x)),
            _ => Err(Error::NotImplemented),
        },
        CommandType::SRSP => match header.command.id {
            0x02 => srsp::Init::try_decode(cursor).map(|x| MAC_Init_SRSP(x)),
            0x05 => srsp::DataReq::try_decode(cursor).map(|x| MAC_DataReq_SRSP(x)),
            0x0e => srsp::PurgeReq::try_decode(cursor).map(|x| MAC_PurgeReq_SRSP(x)),
            0x06 => srsp::AssociateReq::try_decode(cursor).map(|x| MAC_AssociateReq_SRSP(x)),
            0x50 => srsp::AssociateRsp::try_decode(cursor).map(|x| MAC_AssociateRsp_SRSP(x)),
            0x07 => srsp::DisassociateReq::try_decode(cursor).map(|x| MAC_DisassociateReq_SRSP(x)),
            0x08 => srsp::GetReq::try_decode(cursor).map(|x| MAC_GetReq_SRSP(x)),
            0x09 => srsp::SetReq::try_decode(cursor).map(|x| MAC_SetReq_SRSP(x)),
            0x30 => srsp::SecurityGetReq::try_decode(cursor).map(|x| MAC_SecurityGetReq_SRSP(x)),
            0x31 => srsp::SecuritySetReq::try_decode(cursor).map(|x| MAC_SecuritySetReq_SRSP(x)),
            0x32 => srsp::UpdatePANIdReq::try_decode(cursor).map(|x| MAC_UpdatePANIdReq_SRSP(x)),
            0x33 => srsp::AddDeviceReq::try_decode(cursor).map(|x| MAC_AddDeviceReq_SRSP(x)),
            0x34 => srsp::DeleteDeviceReq::try_decode(cursor).map(|x| MAC_DeleteDeviceReq_SRSP(x)),
            0x35 => srsp::DeleteAllDevicesReq::try_decode(cursor)
                .map(|x| MAC_DeleteAllDevicesReq_SRSP(x)),
            0x36 => srsp::DeleteKeyReq::try_decode(cursor).map(|x| MAC_DeleteKeyReq_SRSP(x)),
            0x37 => srsp::ReadKeyReq::try_decode(cursor).map(|x| MAC_ReadKeyReq_SRSP(x)),
            0x38 => srsp::WriteKeyReq::try_decode(cursor).map(|x| MAC_WriteKeyReq_SRSP(x)),
            0x51 => srsp::OrphanRsp::try_decode(cursor).map(|x| MAC_OrphanRsp_SRSP(x)),
            0x0d => srsp::PollReq::try_decode(cursor).map(|x| MAC_PollReq_SRSP(x)),
            0x01 => srsp::ResetReq::try_decode(cursor).map(|x| MAC_ResetReq_SRSP(x)),
            0x0c => srsp::ScanReq::try_decode(cursor).map(|x| MAC_ScanReq_SRSP(x)),
            0x03 => srsp::StartReq::try_decode(cursor).map(|x| MAC_StartReq_SRSP(x)),
            0x04 => srsp::SyncReq::try_decode(cursor).map(|x| MAC_SyncReq_SRSP(x)),
            0x0f => srsp::SetRxGainReq::try_decode(cursor).map(|x| MAC_SetRxGainReq_SRSP(x)),
            0x44 => srsp::WSAsyncReq::try_decode(cursor).map(|x| MAC_WSAsyncReq_SRSP(x)),
            0x40 => srsp::FHEnableReq::try_decode(cursor).map(|x| MAC_FHEnableReq_SRSP(x)),
            0x41 => srsp::FHStartReq::try_decode(cursor).map(|x| MAC_FHStartReq_SRSP(x)),
            0x42 => srsp::FHGetReq::try_decode(cursor).map(|x| MAC_FHGetReq_SRSP(x)),
            0x43 => srsp::FHSetReq::try_decode(cursor).map(|x| MAC_FHSetReq_SRSP(x)),
            _ => Err(Error::NotImplemented),
        },
    }
}
