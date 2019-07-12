#[derive(Debug)]
pub enum Error {
    InvalidStartOfFrame(u8),
    InvalidFrameCheckSequence(Vec<u8>),
    InvalidCommandType(u8),
    InvalidSubsystem(u8),
    InvalidExtendedHeaderStatus(u8),
    InvalidErrorCode(u8),
    InvalidAddressMode(u8),
    InvalidTxOption(u8),
    InvalidSecurityLevel(u8),
    InvalidKeyIdMode(u8),
    InvalidFrameType(u8),
    InvalidAssociationStatus(u8),
    InvalidStatus(u8),
    NotEnoughBytes,
    NotImplemented,
}
