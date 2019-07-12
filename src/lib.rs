pub mod error;
pub mod frame;
pub mod parser;
pub mod subsystem;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::{frame, subsystem};
    use std::convert::TryFrom;
    use std::io::Cursor;

    #[test]
    fn parse_mt_header_1() {
        let data = [0x00, 0x01, 0x02];
        let mut cursor = Cursor::new(&data[..]);
        let header = frame::MTHeader::try_from(&mut cursor).unwrap();
        assert_eq!(header.length, 0);
        assert_eq!(header.has_extension(), false);
        assert_eq!(header.command.cmd_type, frame::CommandType::POLL);
        assert_eq!(header.command.subsystem, subsystem::MTSubsystem::SYS);
        assert_eq!(header.command.id, 2);
    }

    #[test]
    fn parse_mt_header_2() {
        let data = [0xFF, 0x81, 0x0A];
        let mut cursor = Cursor::new(&data[..]);
        let header = frame::MTHeader::try_from(&mut cursor).unwrap();
        assert_eq!(header.length, 255);
        assert_eq!(header.has_extension(), true);
        assert_eq!(header.command.cmd_type, frame::CommandType::POLL);
        assert_eq!(header.command.subsystem, subsystem::MTSubsystem::SYS);
        assert_eq!(header.command.id, 10);
    }
}
