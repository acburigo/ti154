use crate::error::Error;
use crate::frame::{MTFrame, MTHeader};
use std::io::Cursor;

const START_OF_FRAME_TOKEN: u8 = 0xfe;

pub trait ToBytes {
    fn to_bytes(s: Self) -> Vec<u8>;
}

enum State {
    WaitingStartOfFrame,
    GatheringMTFrameBytes,
    WaitingFrameCheckSequence,
}

pub struct Parser {
    buffer: Vec<u8>,
    state: State,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            buffer: Vec::new(),
            state: State::WaitingStartOfFrame,
        }
    }

    pub fn feed(&mut self, new_byte: u8) -> Option<Result<MTFrame, Error>> {
        match self.state {
            State::WaitingStartOfFrame => {
                if new_byte != START_OF_FRAME_TOKEN {
                    return Some(Err(Error::InvalidStartOfFrame(new_byte)));
                }
                self.state = State::GatheringMTFrameBytes;
            }

            State::GatheringMTFrameBytes => {
                self.buffer.push(new_byte);

                let payload_length = self.buffer[0] as usize;
                let expected_frame_size = MTHeader::size() + payload_length;

                if self.buffer.len() >= expected_frame_size {
                    self.state = State::WaitingFrameCheckSequence;
                }
            }

            State::WaitingFrameCheckSequence => {
                let fcs = Self::compute_frame_check_sequence(self.buffer.as_slice());

                let result = if fcs == new_byte {
                    self.parse_frame()
                } else {
                    Err(Error::InvalidFrameCheckSequence(self.buffer.clone()))
                };

                self.reset();
                return Some(result);
            }
        }

        None
    }

    pub fn reset(&mut self) {
        self.buffer.clear();
        self.state = State::WaitingStartOfFrame;
    }

    fn parse_frame(&self) -> Result<MTFrame, Error> {
        let mut cursor = Cursor::new(self.buffer.as_slice());
        MTFrame::try_decode(&mut cursor)
    }

    fn compute_frame_check_sequence(mt_frame_bytes: &[u8]) -> u8 {
        mt_frame_bytes.iter().fold(0, |acc, x| acc ^ x)
    }
}
