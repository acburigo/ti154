pub mod error;
pub mod frame;
pub mod parser;
pub mod subsystem;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::{frame, types};
    use std::convert::TryFrom;
    use std::io::Cursor;

    #[test]
    fn parse_mt_header_1() {
        let data = [0x00, 0x01, 0x02];
        let mut cursor = Cursor::new(&data[..]);
        let header = frame::MTHeader::try_from(&mut cursor).unwrap();
        assert_eq!(header.length, 0);
        assert_eq!(header.has_extension(), false);
        assert_eq!(header.command.cmd_type, types::CommandType::POLL);
        assert_eq!(header.command.subsystem, types::MTSubsystem::SYS);
        assert_eq!(header.command.id, 2);
    }

    #[test]
    fn parse_mt_header_2() {
        let data = [0xFF, 0x81, 0x0A];
        let mut cursor = Cursor::new(&data[..]);
        let header = frame::MTHeader::try_from(&mut cursor).unwrap();
        assert_eq!(header.length, 255);
        assert_eq!(header.has_extension(), true);
        assert_eq!(header.command.cmd_type, types::CommandType::POLL);
        assert_eq!(header.command.subsystem, types::MTSubsystem::SYS);
        assert_eq!(header.command.id, 10);
    }

    #[test]
    fn parse_sys_reset_ind() {
        let data = [0x6, 0x41, 0x80, 0x0, 0x3, 0x1, 0x2, 0x2, 0x0];
        let mut cursor = Cursor::new(&data[..]);
        let frame = frame::MTFrame::try_from(&mut cursor).unwrap();
        assert_eq!(frame.header.length, 0x06);
        assert_eq!(frame.header.has_extension(), false);
        assert_eq!(frame.header.command.cmd_type, types::CommandType::AREQ);
        assert_eq!(frame.header.command.subsystem, types::MTSubsystem::SYS);
        assert_eq!(frame.header.command.id, 0x80);
        assert!(frame.extended_header.is_none());
        println!("{:#?}", frame);
    }
}
