pub mod areq;
pub mod sreq;
pub mod srsp;

use crate::error::Error;
use crate::frame::{MTExtendedHeader, MTHeader};
use crate::subsystem::MTFramePayload;
use crate::types::CommandType;
use std::io::Cursor;

pub fn try_from(
    header: &MTHeader,
    _: &Option<MTExtendedHeader>,
    cursor: &mut Cursor<&[u8]>,
) -> Result<MTFramePayload, Error> {
    use MTFramePayload::*;

    match header.command.cmd_type {
        CommandType::POLL => Err(Error::NotImplemented),
        CommandType::SREQ => match header.command.id {
            0x02 => Ok(MAC_Init_SREQ(sreq::Init::try_from(cursor)?)),
            0x05 => Ok(MAC_DataReq_SREQ(sreq::DataReq::try_from(cursor)?)),
            0x0e => Ok(MAC_PurgeReq_SREQ(sreq::PurgeReq::try_from(cursor)?)),
            0x06 => Ok(MAC_AssociateReq_SREQ(sreq::AssociateReq::try_from(cursor)?)),
            0x50 => Ok(MAC_AssociateRsp_SREQ(sreq::AssociateRsp::try_from(cursor)?)),
            0x07 => Ok(MAC_DisassociateReq_SREQ(sreq::DisassociateReq::try_from(
                cursor,
            )?)),
            0x08 => Ok(MAC_GetReq_SREQ(sreq::GetReq::try_from(cursor)?)),
            0x09 => Ok(MAC_SetReq_SREQ(sreq::SetReq::try_from(cursor)?)),
            0x30 => Ok(MAC_SecurityGetReq_SREQ(sreq::SecurityGetReq::try_from(
                cursor,
            )?)),
            0x31 => Ok(MAC_SecuritySetReq_SREQ(sreq::SecuritySetReq::try_from(
                cursor,
            )?)),
            0x32 => Ok(MAC_UpdatePANIdReq_SREQ(sreq::UpdatePANIdReq::try_from(
                cursor,
            )?)),
            0x33 => Ok(MAC_AddDeviceReq_SREQ(sreq::AddDeviceReq::try_from(cursor)?)),
            0x34 => Ok(MAC_DeleteDeviceReq_SREQ(sreq::DeleteDeviceReq::try_from(
                cursor,
            )?)),
            0x35 => Ok(MAC_DeleteAllDevicesReq_SREQ(
                sreq::DeleteAllDevicesReq::try_from(cursor)?,
            )),
            0x36 => Ok(MAC_DeleteKeyReq_SREQ(sreq::DeleteKeyReq::try_from(cursor)?)),
            0x37 => Ok(MAC_ReadKeyReq_SREQ(sreq::ReadKeyReq::try_from(cursor)?)),
            0x38 => Ok(MAC_WriteKeyReq_SREQ(sreq::WriteKeyReq::try_from(cursor)?)),
            0x51 => Ok(MAC_OrphanRsp_SREQ(sreq::OrphanRsp::try_from(cursor)?)),
            0x0d => Ok(MAC_PollReq_SREQ(sreq::PollReq::try_from(cursor)?)),
            0x01 => Ok(MAC_ResetReq_SREQ(sreq::ResetReq::try_from(cursor)?)),
            0x0c => Ok(MAC_ScanReq_SREQ(sreq::ScanReq::try_from(cursor)?)),
            0x03 => Ok(MAC_StartReq_SREQ(sreq::StartReq::try_from(cursor)?)),
            0x04 => Ok(MAC_SyncReq_SREQ(sreq::SyncReq::try_from(cursor)?)),
            0x0f => Ok(MAC_SetRxGainReq_SREQ(sreq::SetRxGainReq::try_from(cursor)?)),
            0x44 => Ok(MAC_WSAsyncReq_SREQ(sreq::WSAsyncReq::try_from(cursor)?)),
            0x40 => Ok(MAC_FHEnableReq_SREQ(sreq::FHEnableReq::try_from(cursor)?)),
            0x41 => Ok(MAC_FHStartReq_SREQ(sreq::FHStartReq::try_from(cursor)?)),
            0x42 => Ok(MAC_FHGetReq_SREQ(sreq::FHGetReq::try_from(cursor)?)),
            0x43 => Ok(MAC_FHSetReq_SREQ(sreq::FHSetReq::try_from(cursor)?)),
            _ => Err(Error::NotImplemented),
        },
        CommandType::AREQ => match header.command.id {
            0x84 => Ok(MAC_DataCnf_AREQ(areq::DataCnf::try_from(cursor)?)),
            0x85 => Ok(MAC_DataInd_AREQ(areq::DataInd::try_from(cursor)?)),
            0x90 => Ok(MAC_PurgeCnf_AREQ(areq::PurgeCnf::try_from(cursor)?)),
            0x93 => Ok(MAC_WSAsyncInd_AREQ(areq::WSAsyncInd::try_from(cursor)?)),
            0x80 => Ok(MAC_SyncLossInd_AREQ(areq::SyncLossInd::try_from(cursor)?)),
            0x81 => Ok(MAC_AssociateInd_AREQ(areq::AssociateInd::try_from(cursor)?)),
            0x82 => Ok(MAC_AssociateCnf_AREQ(areq::AssociateCnf::try_from(cursor)?)),
            0x83 => Ok(MAC_BeaconNotifyInd_AREQ(areq::BeaconNotifyInd::try_from(
                cursor,
            )?)),
            0x86 => Ok(MAC_DisassociateInd_AREQ(areq::DisassociateInd::try_from(
                cursor,
            )?)),
            0x87 => Ok(MAC_DisassociateCnf_AREQ(areq::DisassociateCnf::try_from(
                cursor,
            )?)),
            0x8a => Ok(MAC_OrphanInd_AREQ(areq::OrphanInd::try_from(cursor)?)),
            0x8b => Ok(MAC_PollCnf_AREQ(areq::PollCnf::try_from(cursor)?)),
            0x91 => Ok(MAC_PollInd_AREQ(areq::PollInd::try_from(cursor)?)),
            0x8c => Ok(MAC_ScanCnf_AREQ(areq::ScanCnf::try_from(cursor)?)),
            0x8d => Ok(MAC_CommStatusInd_AREQ(areq::CommStatusInd::try_from(
                cursor,
            )?)),
            0x8e => Ok(MAC_StartCnf_AREQ(areq::StartCnf::try_from(cursor)?)),
            0x92 => Ok(MAC_WSAsyncCnf_AREQ(areq::WSAsyncCnf::try_from(cursor)?)),
            _ => Err(Error::NotImplemented),
        },
        CommandType::SRSP => match header.command.id {
            0x02 => Ok(MAC_Init_SRSP(srsp::Init::try_from(cursor)?)),
            0x05 => Ok(MAC_DataReq_SRSP(srsp::DataReq::try_from(cursor)?)),
            0x0e => Ok(MAC_PurgeReq_SRSP(srsp::PurgeReq::try_from(cursor)?)),
            0x06 => Ok(MAC_AssociateReq_SRSP(srsp::AssociateReq::try_from(cursor)?)),
            0x50 => Ok(MAC_AssociateRsp_SRSP(srsp::AssociateRsp::try_from(cursor)?)),
            0x07 => Ok(MAC_DisassociateReq_SRSP(srsp::DisassociateReq::try_from(
                cursor,
            )?)),
            0x08 => Ok(MAC_GetReq_SRSP(srsp::GetReq::try_from(cursor)?)),
            0x09 => Ok(MAC_SetReq_SRSP(srsp::SetReq::try_from(cursor)?)),
            0x30 => Ok(MAC_SecurityGetReq_SRSP(srsp::SecurityGetReq::try_from(
                cursor,
            )?)),
            0x31 => Ok(MAC_SecuritySetReq_SRSP(srsp::SecuritySetReq::try_from(
                cursor,
            )?)),
            0x32 => Ok(MAC_UpdatePANIdReq_SRSP(srsp::UpdatePANIdReq::try_from(
                cursor,
            )?)),
            0x33 => Ok(MAC_AddDeviceReq_SRSP(srsp::AddDeviceReq::try_from(cursor)?)),
            0x34 => Ok(MAC_DeleteDeviceReq_SRSP(srsp::DeleteDeviceReq::try_from(
                cursor,
            )?)),
            0x35 => Ok(MAC_DeleteAllDevicesReq_SRSP(
                srsp::DeleteAllDevicesReq::try_from(cursor)?,
            )),
            0x36 => Ok(MAC_DeleteKeyReq_SRSP(srsp::DeleteKeyReq::try_from(cursor)?)),
            0x37 => Ok(MAC_ReadKeyReq_SRSP(srsp::ReadKeyReq::try_from(cursor)?)),
            0x38 => Ok(MAC_WriteKeyReq_SRSP(srsp::WriteKeyReq::try_from(cursor)?)),
            0x51 => Ok(MAC_OrphanRsp_SRSP(srsp::OrphanRsp::try_from(cursor)?)),
            0x0d => Ok(MAC_PollReq_SRSP(srsp::PollReq::try_from(cursor)?)),
            0x01 => Ok(MAC_ResetReq_SRSP(srsp::ResetReq::try_from(cursor)?)),
            0x0c => Ok(MAC_ScanReq_SRSP(srsp::ScanReq::try_from(cursor)?)),
            0x03 => Ok(MAC_StartReq_SRSP(srsp::StartReq::try_from(cursor)?)),
            0x04 => Ok(MAC_SyncReq_SRSP(srsp::SyncReq::try_from(cursor)?)),
            0x0f => Ok(MAC_SetRxGainReq_SRSP(srsp::SetRxGainReq::try_from(cursor)?)),
            0x44 => Ok(MAC_WSAsyncReq_SRSP(srsp::WSAsyncReq::try_from(cursor)?)),
            0x40 => Ok(MAC_FHEnableReq_SRSP(srsp::FHEnableReq::try_from(cursor)?)),
            0x41 => Ok(MAC_FHStartReq_SRSP(srsp::FHStartReq::try_from(cursor)?)),
            0x42 => Ok(MAC_FHGetReq_SRSP(srsp::FHGetReq::try_from(cursor)?)),
            0x43 => Ok(MAC_FHSetReq_SRSP(srsp::FHSetReq::try_from(cursor)?)),
            _ => Err(Error::NotImplemented),
        },
    }
}
