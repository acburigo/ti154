pub mod areq;
pub mod sreq;
pub mod srsp;

use crate::error::Error;
use crate::frame::MTHeader;
use crate::subsystem::MTFramePayload;
use crate::types::CommandType;
use num_traits::FromPrimitive;
use std::io::Cursor;

pub fn try_decode(header: &MTHeader, cursor: &mut Cursor<&[u8]>) -> Result<MTFramePayload, Error> {
    use crate::types::MACCommandId::*;
    use MTFramePayload::*;

    let command_id =
        FromPrimitive::from_u8(header.command.id).ok_or(Error::InvalidStatus(header.command.id))?;

    match header.command.cmd_type {
        CommandType::POLL => Err(Error::NotImplemented),
        CommandType::SREQ => match command_id {
            Init => sreq::Init::try_decode(cursor).map(|x| MAC_Init_SREQ(x)),
            DataReq => sreq::DataReq::try_decode(cursor).map(|x| MAC_DataReq_SREQ(x)),
            PurgeReq => sreq::PurgeReq::try_decode(cursor).map(|x| MAC_PurgeReq_SREQ(x)),
            AssociateReq => {
                sreq::AssociateReq::try_decode(cursor).map(|x| MAC_AssociateReq_SREQ(x))
            }
            AssociateRsp => {
                sreq::AssociateRsp::try_decode(cursor).map(|x| MAC_AssociateRsp_SREQ(x))
            }
            DisassociateReq => {
                sreq::DisassociateReq::try_decode(cursor).map(|x| MAC_DisassociateReq_SREQ(x))
            }
            GetReq => sreq::GetReq::try_decode(cursor).map(|x| MAC_GetReq_SREQ(x)),
            SetReq => sreq::SetReq::try_decode(cursor).map(|x| MAC_SetReq_SREQ(x)),
            SecurityGetReq => {
                sreq::SecurityGetReq::try_decode(cursor).map(|x| MAC_SecurityGetReq_SREQ(x))
            }
            SecuritySetReq => {
                sreq::SecuritySetReq::try_decode(cursor).map(|x| MAC_SecuritySetReq_SREQ(x))
            }
            UpdatePANIdReq => {
                sreq::UpdatePANIdReq::try_decode(cursor).map(|x| MAC_UpdatePANIdReq_SREQ(x))
            }
            AddDeviceReq => {
                sreq::AddDeviceReq::try_decode(cursor).map(|x| MAC_AddDeviceReq_SREQ(x))
            }
            DeleteDeviceReq => {
                sreq::DeleteDeviceReq::try_decode(cursor).map(|x| MAC_DeleteDeviceReq_SREQ(x))
            }
            DeleteAllDevicesReq => sreq::DeleteAllDevicesReq::try_decode(cursor)
                .map(|x| MAC_DeleteAllDevicesReq_SREQ(x)),
            DeleteKeyReq => {
                sreq::DeleteKeyReq::try_decode(cursor).map(|x| MAC_DeleteKeyReq_SREQ(x))
            }
            ReadKeyReq => sreq::ReadKeyReq::try_decode(cursor).map(|x| MAC_ReadKeyReq_SREQ(x)),
            WriteKeyReq => sreq::WriteKeyReq::try_decode(cursor).map(|x| MAC_WriteKeyReq_SREQ(x)),
            OrphanRsp => sreq::OrphanRsp::try_decode(cursor).map(|x| MAC_OrphanRsp_SREQ(x)),
            PollReq => sreq::PollReq::try_decode(cursor).map(|x| MAC_PollReq_SREQ(x)),
            ResetReq => sreq::ResetReq::try_decode(cursor).map(|x| MAC_ResetReq_SREQ(x)),
            ScanReq => sreq::ScanReq::try_decode(cursor).map(|x| MAC_ScanReq_SREQ(x)),
            StartReq => sreq::StartReq::try_decode(cursor).map(|x| MAC_StartReq_SREQ(x)),
            SyncReq => sreq::SyncReq::try_decode(cursor).map(|x| MAC_SyncReq_SREQ(x)),
            SetRxGainReq => {
                sreq::SetRxGainReq::try_decode(cursor).map(|x| MAC_SetRxGainReq_SREQ(x))
            }
            WSAsyncReq => sreq::WSAsyncReq::try_decode(cursor).map(|x| MAC_WSAsyncReq_SREQ(x)),
            FHEnableReq => sreq::FHEnableReq::try_decode(cursor).map(|x| MAC_FHEnableReq_SREQ(x)),
            FHStartReq => sreq::FHStartReq::try_decode(cursor).map(|x| MAC_FHStartReq_SREQ(x)),
            FHGetReq => sreq::FHGetReq::try_decode(cursor).map(|x| MAC_FHGetReq_SREQ(x)),
            FHSetReq => sreq::FHSetReq::try_decode(cursor).map(|x| MAC_FHSetReq_SREQ(x)),
            _ => Err(Error::NotImplemented),
        },
        CommandType::AREQ => match command_id {
            DataCnf => areq::DataCnf::try_decode(cursor).map(|x| MAC_DataCnf_AREQ(x)),
            DataInd => areq::DataInd::try_decode(cursor).map(|x| MAC_DataInd_AREQ(x)),
            PurgeCnf => areq::PurgeCnf::try_decode(cursor).map(|x| MAC_PurgeCnf_AREQ(x)),
            WSAsyncInd => areq::WSAsyncInd::try_decode(cursor).map(|x| MAC_WSAsyncInd_AREQ(x)),
            SyncLossInd => areq::SyncLossInd::try_decode(cursor).map(|x| MAC_SyncLossInd_AREQ(x)),
            AssociateInd => {
                areq::AssociateInd::try_decode(cursor).map(|x| MAC_AssociateInd_AREQ(x))
            }
            AssociateCnf => {
                areq::AssociateCnf::try_decode(cursor).map(|x| MAC_AssociateCnf_AREQ(x))
            }
            BeaconNotifyInd => {
                areq::BeaconNotifyInd::try_decode(cursor).map(|x| MAC_BeaconNotifyInd_AREQ(x))
            }
            DisassociateInd => {
                areq::DisassociateInd::try_decode(cursor).map(|x| MAC_DisassociateInd_AREQ(x))
            }
            DisassociateCnf => {
                areq::DisassociateCnf::try_decode(cursor).map(|x| MAC_DisassociateCnf_AREQ(x))
            }
            OrphanInd => areq::OrphanInd::try_decode(cursor).map(|x| MAC_OrphanInd_AREQ(x)),
            PollCnf => areq::PollCnf::try_decode(cursor).map(|x| MAC_PollCnf_AREQ(x)),
            PollInd => areq::PollInd::try_decode(cursor).map(|x| MAC_PollInd_AREQ(x)),
            ScanCnf => areq::ScanCnf::try_decode(cursor).map(|x| MAC_ScanCnf_AREQ(x)),
            CommStatusInd => {
                areq::CommStatusInd::try_decode(cursor).map(|x| MAC_CommStatusInd_AREQ(x))
            }
            StartCnf => areq::StartCnf::try_decode(cursor).map(|x| MAC_StartCnf_AREQ(x)),
            WSAsyncCnf => areq::WSAsyncCnf::try_decode(cursor).map(|x| MAC_WSAsyncCnf_AREQ(x)),
            _ => Err(Error::NotImplemented),
        },
        CommandType::SRSP => match command_id {
            Init => srsp::Init::try_decode(cursor).map(|x| MAC_Init_SRSP(x)),
            DataReq => srsp::DataReq::try_decode(cursor).map(|x| MAC_DataReq_SRSP(x)),
            PurgeReq => srsp::PurgeReq::try_decode(cursor).map(|x| MAC_PurgeReq_SRSP(x)),
            AssociateReq => {
                srsp::AssociateReq::try_decode(cursor).map(|x| MAC_AssociateReq_SRSP(x))
            }
            AssociateRsp => {
                srsp::AssociateRsp::try_decode(cursor).map(|x| MAC_AssociateRsp_SRSP(x))
            }
            DisassociateReq => {
                srsp::DisassociateReq::try_decode(cursor).map(|x| MAC_DisassociateReq_SRSP(x))
            }
            GetReq => srsp::GetReq::try_decode(cursor).map(|x| MAC_GetReq_SRSP(x)),
            SetReq => srsp::SetReq::try_decode(cursor).map(|x| MAC_SetReq_SRSP(x)),
            SecurityGetReq => {
                srsp::SecurityGetReq::try_decode(cursor).map(|x| MAC_SecurityGetReq_SRSP(x))
            }
            SecuritySetReq => {
                srsp::SecuritySetReq::try_decode(cursor).map(|x| MAC_SecuritySetReq_SRSP(x))
            }
            UpdatePANIdReq => {
                srsp::UpdatePANIdReq::try_decode(cursor).map(|x| MAC_UpdatePANIdReq_SRSP(x))
            }
            AddDeviceReq => {
                srsp::AddDeviceReq::try_decode(cursor).map(|x| MAC_AddDeviceReq_SRSP(x))
            }
            DeleteDeviceReq => {
                srsp::DeleteDeviceReq::try_decode(cursor).map(|x| MAC_DeleteDeviceReq_SRSP(x))
            }
            DeleteAllDevicesReq => srsp::DeleteAllDevicesReq::try_decode(cursor)
                .map(|x| MAC_DeleteAllDevicesReq_SRSP(x)),
            DeleteKeyReq => {
                srsp::DeleteKeyReq::try_decode(cursor).map(|x| MAC_DeleteKeyReq_SRSP(x))
            }
            ReadKeyReq => srsp::ReadKeyReq::try_decode(cursor).map(|x| MAC_ReadKeyReq_SRSP(x)),
            WriteKeyReq => srsp::WriteKeyReq::try_decode(cursor).map(|x| MAC_WriteKeyReq_SRSP(x)),
            OrphanRsp => srsp::OrphanRsp::try_decode(cursor).map(|x| MAC_OrphanRsp_SRSP(x)),
            PollReq => srsp::PollReq::try_decode(cursor).map(|x| MAC_PollReq_SRSP(x)),
            ResetReq => srsp::ResetReq::try_decode(cursor).map(|x| MAC_ResetReq_SRSP(x)),
            ScanReq => srsp::ScanReq::try_decode(cursor).map(|x| MAC_ScanReq_SRSP(x)),
            StartReq => srsp::StartReq::try_decode(cursor).map(|x| MAC_StartReq_SRSP(x)),
            SyncReq => srsp::SyncReq::try_decode(cursor).map(|x| MAC_SyncReq_SRSP(x)),
            SetRxGainReq => {
                srsp::SetRxGainReq::try_decode(cursor).map(|x| MAC_SetRxGainReq_SRSP(x))
            }
            WSAsyncReq => srsp::WSAsyncReq::try_decode(cursor).map(|x| MAC_WSAsyncReq_SRSP(x)),
            FHEnableReq => srsp::FHEnableReq::try_decode(cursor).map(|x| MAC_FHEnableReq_SRSP(x)),
            FHStartReq => srsp::FHStartReq::try_decode(cursor).map(|x| MAC_FHStartReq_SRSP(x)),
            FHGetReq => srsp::FHGetReq::try_decode(cursor).map(|x| MAC_FHGetReq_SRSP(x)),
            FHSetReq => srsp::FHSetReq::try_decode(cursor).map(|x| MAC_FHSetReq_SRSP(x)),
            _ => Err(Error::NotImplemented),
        },
    }
}
