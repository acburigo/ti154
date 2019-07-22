use crate::error::Error;
use bytes::{Buf, BufMut};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::io::Cursor;
use std::io::Read;

#[allow(non_camel_case_types)]
#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum MACCommandId {
    DataCnf = 0x84,
    DataInd = 0x85,
    PurgeCnf = 0x90,
    WSAsyncInd = 0x93,
    SyncLossInd = 0x80,
    AssociateInd = 0x81,
    AssociateCnf = 0x82,
    BeaconNotifyInd = 0x83,
    DisassociateInd = 0x86,
    DisassociateCnf = 0x87,
    OrphanInd = 0x8a,
    PollCnf = 0x8b,
    PollInd = 0x91,
    ScanCnf = 0x8c,
    CommStatusInd = 0x8d,
    StartCnf = 0x8e,
    WSAsyncCnf = 0x92,
    Init = 0x02,
    DataReq = 0x05,
    PurgeReq = 0x0e,
    AssociateReq = 0x06,
    AssociateRsp = 0x50,
    DisassociateReq = 0x07,
    GetReq = 0x08,
    SetReq = 0x09,
    SecurityGetReq = 0x30,
    SecuritySetReq = 0x31,
    UpdatePANIdReq = 0x32,
    AddDeviceReq = 0x33,
    DeleteDeviceReq = 0x34,
    DeleteAllDevicesReq = 0x35,
    DeleteKeyReq = 0x36,
    ReadKeyReq = 0x37,
    WriteKeyReq = 0x38,
    OrphanRsp = 0x51,
    PollReq = 0x0d,
    ResetReq = 0x01,
    ScanReq = 0x0c,
    StartReq = 0x03,
    SyncReq = 0x04,
    SetRxGainReq = 0x0f,
    WSAsyncReq = 0x44,
    FHEnableReq = 0x40,
    FHStartReq = 0x41,
    FHGetReq = 0x42,
    FHSetReq = 0x43,
}

#[allow(non_camel_case_types)]
#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum RPCCommandId {
    MTCommandError = 0x00,
}

#[allow(non_camel_case_types)]
#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum SYSCommandId {
    ResetReq = 0x00,
    ResetInd = 0x80,
    PingReq = 0x01,
    VersionReq = 0x02,
    NVCreateReq = 0x30,
    NVDeleteReq = 0x31,
    NVLengthReq = 0x32,
    NVReadReq = 0x33,
    NVWriteReq = 0x34,
    NVUpdateReq = 0x35,
    NVCompactReq = 0x36,
}

#[allow(non_camel_case_types)]
#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum UTILCommandId {
    Loopback = 0x10,
    CallbackSubCmd = 0x06,
    GetExtAddr = 0xee,
    Random = 0x12,
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum CommandType {
    POLL = 0,
    SREQ = 1,
    AREQ = 2,
    SRSP = 3,
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum MTExtendedHeaderStatus {
    Success = 0,                           // Success
    ResendLastFrame = 1,                   // Request - resend last frame
    UnsupportedStackId = 2,                // Unsupported Stack ID
    BlockOutOfOrder = 3,                   // Block out of order – fragmentation aborted
    BlockLengthChanged = 4,                // Block length changed – fragmentation aborted
    MemoryAllocationError = 5,             // Memory allocation error – fragmentation aborted
    FragmentationSequenceCompleted = 6,    // Fragmentation sequence completed
    FragmentationSequenceAborted = 7,      // Fragmentation sequence aborted
    UnsupportedFragmentationAckStatus = 8, // Unsupported Fragmentation Ack Status
}

impl MTExtendedHeaderStatus {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidExtendedHeaderStatus(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum MTSubsystem {
    RPC = 0,
    SYS = 1,
    MAC = 2,
    UTIL = 7,
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum Status {
    Success = 0x00,
    Unsupported = 0x18,
    BadState = 0x19,
    NoResources = 0x1A,
    RPCCommandSubsystemError = 0x25,
    RPCCommandIdError = 0x26,
    RPCCommandLengthError = 0x27,
    RPCCommandUnsupportedType = 0x28,
    FHAPIError = 0x61,
    FHAPINotSupportedIE = 0x62,
    FHAPINotInAsync = 0x63,
    FHAPINoEntryInTheNeighbor = 0x64,
    FHAPIOutSlot = 0x65,
    FHAPIInvalidAddress = 0x66,
    FHAPIInvalidFormat = 0x67,
    FHAPINotSupportedPIB = 0x68,
    FHAPIReadOnlyPIB = 0x69,
    FHAPIInvalidParamPIB = 0x6A,
    FHAPIInvalidFrameType = 0x6B,
    FHAPIExpiredNode = 0x6C,
    CounterError = 0xDB,
    ImproperKeyType = 0xDC,
    ImproperSecurityLevel = 0xDD,
    UnsupportedLegacy = 0xDE,
    UnsupportedSecurity = 0xDF,
    BeaconLoss = 0xE0,
    ChannelAccessFailure = 0xE1,
    Denied = 0xE2,
    DisableTRXFailure = 0xE3,
    SecurityError = 0xE4,
    FrameTooLong = 0xE5,
    InvalidGTS = 0xE6,
    InvalidHandle = 0xE7,
    InvalidParameter = 0xE8,
    NoAck = 0xE9,
    NoBeacon = 0xEA,
    NoData = 0xEB,
    NoShortAddress = 0xEC,
    OutOfCAP = 0xED,
    PANIdConflict = 0xEE,
    Realignment = 0xEF,
    TransactionExpired = 0xF0,
    TransactionOverflow = 0xF1,
    TxActive = 0xF2,
    UnavailableKey = 0xF3,
    UnsupportedAttribute = 0xF4,
    InvalidAddress = 0xF5,
    OnTimeTooLong = 0xF6,
    PastTime = 0xF7,
    TrackingOff = 0xF8,
    InvalidIndex = 0xF9,
    LimitReached = 0xFA,
    ReadOnly = 0xFB,
    ScanInProgress = 0xFC,
    SuperframeOverlap = 0xFD,
    AutoAckPendingAllOn = 0xFE,
    AutoAckPendingAllOff = 0xFF,
}

impl Status {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidStatus(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum AddressMode {
    Addr16Bit = 0x02,
    Addr64Bit = 0x03,
}

impl AddressMode {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidAddressMode(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, PartialEq)]
pub struct ShortAddress {
    pub address: u16,
}

impl ShortAddress {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        Ok(ShortAddress {
            address: cursor.get_u16_le(),
        })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u16_le(self.address);
    }
}

#[derive(Debug, PartialEq)]
pub struct ExtendedAddress {
    pub address: [u8; 8],
}

impl ExtendedAddress {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let mut address: [u8; 8] = Default::default();
        cursor
            .read_exact(&mut address)
            .map_err(|_| Error::NotEnoughBytes)?;
        address.reverse();
        Ok(ExtendedAddress { address })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.extend(self.address.iter().rev());
    }
}

#[derive(Debug, PartialEq)]
pub enum Address {
    Addr16Bit(ShortAddress),
    Addr64Bit(ExtendedAddress),
}

impl Address {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let address_mode = AddressMode::try_decode(Read::by_ref(cursor))?;

        let address = match address_mode {
            AddressMode::Addr16Bit => {
                let address = Address::Addr16Bit(ShortAddress::try_decode(cursor)?);
                std::io::BufRead::consume(cursor, 6);
                address
            }
            AddressMode::Addr64Bit => Address::Addr64Bit(ExtendedAddress::try_decode(cursor)?),
        };

        Ok(address)
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        match self {
            Address::Addr16Bit(address) => {
                buffer.put_u8(AddressMode::Addr16Bit as u8);
                address.encode_into(buffer);
            }
            Address::Addr64Bit(address) => {
                buffer.put_u8(AddressMode::Addr64Bit as u8);
                address.encode_into(buffer);
            }
        }
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum TxOption {
    // Non-acknowledged transmission.
    NoAck = 0x00,

    // Acknowledged transmission.
    // The MAC will attempt to retransmit the frame until it is acknowledged.
    Ack = 0x01,

    // GTS transmission (unused)
    GTS = 0x02,

    // Indirect transmission.
    // The MAC will queue the data and wait for the destination device to poll for it.
    // This can only be used by a coordinator device.
    Indirect = 0x04,

    // Force setting of pending bit for direct transmission.
    PendBit = 0x08,

    // This proprietary option prevents the frame from being retransmitted.
    NoRetrans = 0x10,

    // This proprietary option prevents a MAC_DATA_CNF event from being sent for this frame.
    NoCNF = 0x20,

    // Use PIB value MAC_ALT_BE for the minimum backoff exponent.
    AltBE = 0x40,

    // Use the power and channel values in macDataReq_t instead of the PIB values.
    PwrChan = 0x80,
}

impl TxOption {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidTxOption(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum SecurityLevel {
    NoSecurity = 0x00,
    MIC32Auth = 0x01,
    MIC64Auth = 0x02,
    MIC128Auth = 0x03,
    AESEncryption = 0x04,
    AESEncryptionMIC32 = 0x05,
    AESEncryptionMIC64 = 0x06,
    AESEncryptionMIC128 = 0x07,
}

impl SecurityLevel {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidSecurityLevel(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum KeyIdMode {
    NotUsed = 0x00,
    Key1ByteIndex = 0x01,
    Key4ByteIndex = 0x02,
    Key8ByteIndex = 0x03,
}

impl KeyIdMode {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidKeyIdMode(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, PartialEq)]
pub struct KeySource {
    pub key: [u8; 8],
}

impl KeySource {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let mut key: [u8; 8] = Default::default();
        cursor
            .read_exact(&mut key)
            .map_err(|_| Error::NotEnoughBytes)?;
        key.reverse();
        Ok(KeySource { key })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.extend(self.key.iter().rev());
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum WiSUNAsyncFrameType {
    PANAdvert = 0x00,
    PANAdvertSOL = 0x01,
    PANConfig = 0x02,
    PANConfigSOL = 0x03,
    Data = 0x04,
    Ack = 0x05,
    EAPOL = 0x06,
    Invalid = 0xFF,
}

impl WiSUNAsyncFrameType {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidFrameType(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum AssociationStatus {
    Successful = 0x00,
    PANAtCapacity = 0x01,
    PANAccessDenied = 0x02,
}

impl AssociationStatus {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidAssociationStatus(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum DisassociateReason {
    Reserved = 0x00,
    CoorWishesDevLeave = 0x01,
    DevWishesLeave = 0x02,
}

impl DisassociateReason {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidDisassociationReason(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum MACPIBAttributeId {
    AckWaitDuration = 0x40,
    AssociationPermit = 0x41,
    AutoRequest = 0x42,
    BattLifeExt = 0x43,
    BattLeftExtPeriods = 0x44,
    BeaconPayload = 0x45,
    BeaconPayloadLength = 0x46,
    BeaconOrder = 0x47,
    BeaconTxTime = 0x48,
    BSN = 0x49,
    CoordExtendedAddress = 0x4A,
    CoordShortAddress = 0x4B,
    DSN = 0x4C,
    GTSPermit = 0x4D,
    MaxCSMABackoffs = 0x4E,
    MinBE = 0x4F,
    PANId = 0x50,
    PromiscuousMode = 0x51,
    RxOnWhenIdle = 0x52,
    ShortAddress = 0x53,
    SuperframeOrder = 0x54,
    TransactionPersistenceTime = 0x55,
    AssociatedPANCoord = 0x56,
    MaxBE = 0x57,
    FrameTotalWaitTime = 0x58,
    MaxFrameRetries = 0x59,
    ResponseWaitTime = 0x5A,
    SyncSymbolOffset = 0x5B,
    TimestampSupported = 0x5C,
    SecurityEnabled = 0x5D,
    EBSN = 0x5E,
    EBeaconOrder = 0x5F,
    EBeaconOrderNBPAN = 0x60,
    OffsetTimeslot = 0x61,
    IncludeMPMIE = 0x62,
    PhyFSKPreambleLen = 0x63,
    PhyMRFSKSFD = 0x64,
    PhyTransmitPowerSigned = 0xE0,
    LogicalChannel = 0xE1,
    ExtendedAddress = 0xE2,
    AltBE = 0xE3,
    DeviceBeaconOrder = 0xE4,
    RF4CEPowerSavings = 0xE5,
    FrameVersionSupport = 0xE6,
    ChannelPage = 0xE7,
    PhyCurrentDescriptorId = 0xE8,
    FCSType = 0xE9,
}

impl MACPIBAttributeId {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidMACPIBAttributeId(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum FHPIBAttributeId {
    TrackParentEUI = 0x2000,
    BCInterval = 0x2001,
    UCExcludedChannels = 0x2002,
    BCExcludedChannels = 0x2003,
    UCDwellInterval = 0x2004,
    BCDwellInterval = 0x2005,
    ClockDrift = 0x2006,
    TimingAccuracy = 0x2007,
    UCChannelFunction = 0x2008,
    BCChannelFunction = 0x2009,
    UseParentBSIE = 0x200A,
    BrocastSchedId = 0x200B,
    UCFixedChannel = 0x200C,
    BCFixedChannel = 0x200D,
    PANSize = 0x200E,
    RoutingCost = 0x200F,
    RoutingMethod = 0x2010,
    EAPOLReady = 0x2011,
    FANTPSVersion = 0x2012,
    NetName = 0x2013,
    PANVersion = 0x2014,
    GTK0Hash = 0x2015,
    GTK1Hash = 0x2016,
    GTK2Hash = 0x2017,
    GTK3Hash = 0x2018,
    NeighborValidTime = 0x2019,
}

impl FHPIBAttributeId {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u16_le();
        FromPrimitive::from_u16(value).ok_or(Error::InvalidFHPIBAttributeId(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u16_le(*self as u16);
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum SecurityPIBAttributeId {
    KeyTable = 0x71,
    KeyTableEntries = 0x81,
    DeviceTableEntries = 0x82,
    SecurityLevelTableEntries = 0x83,
    FrameCounter = 0x84,
    AutoRequestSecurityLevel = 0x85,
    AutoRequestKeyIdMode = 0x86,
    AutoRequestKeySource = 0x87,
    AutoRequestKeyIndex = 0x88,
    DefaultKeySource = 0x89,
    PANCoordExtendedAddress = 0x8A,
    PANCoordShortAddress = 0x8B,
    KeyIdLookupEntry = 0xD0,
    KeyIdDeviceEntry = 0xD1,
    KeyIdUsageEntry = 0xD2,
    KeyEntry = 0xD3,
    DeviceEntry = 0xD4,
    SecurityLevelEntry = 0xD5,
}

impl SecurityPIBAttributeId {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidSecurityPIBAttributeId(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum ScanType {
    EnergyDetect = 0x00,
    Active = 0x01,
    Passive = 0x02,
    Orphan = 0x03,
    Active2 = 0x05,
}

impl ScanType {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidScanType(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum PhyId {
    STD_US_915_PHY_1 = 0x01,
    STD_ETSI_863_PHY_3 = 0x03,
    GENERIC_CHINA_433_PHY_128 = 128,
    GENERIC_US_LRM_915_PHY_129 = 129,
    GENERIC_CHINA_LRM_433_PHY_130 = 130,
    GENERIC_ETSI_LRM_863_PHY_131 = 131,
    GENERIC_US_915_PHY_132 = 132,
    GENERIC_ETSI_863_PHY_133 = 133,
    PHY_ID_NONE = 0,
    MRFSK_GENERIC_PHY_ID_BEGIN = 0x04,
    MRFSK_GENERIC_PHY_ID_END = 0x06,
}

impl PhyId {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidPhyId(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum PermitJoin {
    AllBeaconRequests = 0x00,
    OnlyIfPermitJoinIsEnabled = 0x01,
}

impl PermitJoin {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidPhyId(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum MPMScan {
    Disabled = 0x00,
    Enabled = 0x01,
}

impl MPMScan {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidPhyId(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum MPMType {
    BPAN = 0x01,  // Beacon Enabled
    NBPAN = 0x02, // Non-beacon Enabled
}

impl MPMType {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidPhyId(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum WiSUNAsyncOperation {
    Start = 0x00,
    Stop = 0x01,
}

impl WiSUNAsyncOperation {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidPhyId(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum CommEventReason {
    AssociateRsp = 0x00, // Event sent in response to MAC_AssociateRsp().
    OrphanRsp = 0x01,    // Event sent in response to MAC_OrphanRsp().
    RxSecure = 0x02,     // Event sent as a result of receiving a secure frame.
}

impl CommEventReason {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidPhyId(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum ResetType {
    Hard = 0,
    Soft = 1,
}

impl ResetType {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidPhyId(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum TransportProtocolRevision {
    StandardRPCFrame = 2, // Standard RPC frame, no fragmentation
    ExtendedRPCFrame = 3, // Extended RPC frame, fragmentation
}

impl TransportProtocolRevision {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidPhyId(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum ProductIdCode {
    ZStack = 0,
    TI154Stack = 1,
}

impl ProductIdCode {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidPhyId(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum ResetReason {
    Hardware = 0,
    HostRequest = 1,
    HALAssert = 2,
    MACAssert = 3,
    RTOSAssert = 4,
}

impl ResetReason {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidPhyId(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum SubsystemId {
    Sys = 0x01,
    MAC = 0x02,
    Util = 0x07,
    AllSubsystems = 0xFF,
}

impl SubsystemId {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidPhyId(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum ExtendedAddressType {
    DEVICE_MAC_PIB = 0x00,
    DEVICE_PRIMARY = 0x01,
    DEVICE_USER_CCFG = 0x02,
    UNKNOWN = 0xFF,
}

impl ExtendedAddressType {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidPhyId(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}

#[derive(Debug, PartialEq)]
pub struct ChannelsBitMap {
    pub channels: [u8; 17],
}

impl ChannelsBitMap {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let mut channels: [u8; 17] = Default::default();
        cursor
            .read_exact(&mut channels)
            .map_err(|_| Error::NotEnoughBytes)?;
        channels.reverse();
        Ok(ChannelsBitMap { channels })
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.extend(self.channels.iter().rev());
    }
}

#[derive(Debug, FromPrimitive, Copy, Clone)]
pub enum ErrorCode {
    InvalidSubsystem = 0x01,
    InvalidCommandId = 0x02,
    InvalidParameter = 0x03,
    InvalidLength = 0x04,
    UnsupportedExtendedHeaderType = 0x05,
    MemoryAllocationFailure = 0x06,
}

impl ErrorCode {
    pub fn try_decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let value = cursor.get_u8();
        FromPrimitive::from_u8(value).ok_or(Error::InvalidErrorCode(value))
    }

    pub fn encode_into(&self, buffer: &mut Vec<u8>) {
        buffer.put_u8(*self as u8);
    }
}
