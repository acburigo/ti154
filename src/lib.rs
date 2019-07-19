pub mod error;
pub mod frame;
pub mod parser;
pub mod subsystem;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::{frame, subsystem, types};
    use std::io::Cursor;

    #[test]
    fn decode_mt_header_1() {
        let data = [0x00, 0x01, 0x02];
        let mut cursor = Cursor::new(&data[..]);
        let header = frame::MTHeader::try_decode(&mut cursor).unwrap();
        assert_eq!(header.length, 0);
        assert_eq!(header.has_extension(), false);
        assert_eq!(header.command.cmd_type, types::CommandType::POLL);
        assert_eq!(header.command.subsystem, types::MTSubsystem::SYS);
        assert_eq!(header.command.id, 2);
    }

    #[test]
    fn decode_mt_header_2() {
        let data = [0xFF, 0x81, 0x0A];
        let mut cursor = Cursor::new(&data[..]);
        let header = frame::MTHeader::try_decode(&mut cursor).unwrap();
        assert_eq!(header.length, 255);
        assert_eq!(header.has_extension(), true);
        assert_eq!(header.command.cmd_type, types::CommandType::POLL);
        assert_eq!(header.command.subsystem, types::MTSubsystem::SYS);
        assert_eq!(header.command.id, 10);
    }

    #[test]
    fn decode_sys_reset_ind() {
        let data = [0x6, 0x41, 0x80, 0x0, 0x3, 0x1, 0x2, 0x2, 0x0];
        let mut cursor = Cursor::new(&data[..]);
        let frame = frame::MTFrame::try_decode(&mut cursor).unwrap();
        assert_eq!(frame.header.length, 0x06);
        assert_eq!(frame.header.has_extension(), false);
        assert_eq!(frame.header.command.cmd_type, types::CommandType::AREQ);
        assert_eq!(frame.header.command.subsystem, types::MTSubsystem::SYS);
        assert_eq!(frame.header.command.id, 0x80);
        assert!(frame.extended_header.is_none());

        if let subsystem::MTFramePayload::SYS_ResetInd_AREQ(ref payload) = frame.payload {
            assert_eq!(payload.reason, types::ResetReason::Hardware);
            assert_eq!(
                payload.transport,
                types::TransportProtocolRevision::ExtendedRPCFrame
            );
            assert_eq!(payload.product, types::ProductIdCode::TI154Stack);
            assert_eq!(payload.major, 0x02);
            assert_eq!(payload.minor, 0x02);
            assert_eq!(payload.maint, 0x00);
        } else {
            panic!("Incorrect payload type.");
        }
    }

    #[test]
    fn decode_mac_reset_req_srsp() {
        let data = [0x1, 0x62, 0x1, 0x0];
        let mut cursor = Cursor::new(&data[..]);
        let frame = frame::MTFrame::try_decode(&mut cursor).unwrap();
        assert_eq!(frame.header.length, 0x01);
        assert_eq!(frame.header.has_extension(), false);
        assert_eq!(frame.header.command.cmd_type, types::CommandType::SRSP);
        assert_eq!(frame.header.command.subsystem, types::MTSubsystem::MAC);
        assert_eq!(frame.header.command.id, 0x01);
        assert!(frame.extended_header.is_none());

        if let subsystem::MTFramePayload::MAC_ResetReq_SRSP(ref payload) = frame.payload {
            assert_eq!(payload.status, types::Status::Success);
        } else {
            panic!("Incorrect payload type.");
        }
    }

    #[test]
    fn decode_encode_mac_reset_req_srsp() {
        // Decode
        let data = [0x1, 0x62, 0x1, 0x0];
        let mut cursor = Cursor::new(&data[..]);
        let frame = frame::MTFrame::try_decode(&mut cursor).unwrap();

        // Encode
        let mut buffer = Vec::new();
        frame.encode_into(&mut buffer);

        // Check
        assert_eq!(buffer, data);
    }
}
