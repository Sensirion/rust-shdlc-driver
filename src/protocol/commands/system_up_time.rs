use crate::protocol::commands::ShdlcCommand;
use crate::protocol::errors::ShdlcError;
use byteorder::{BigEndian, ByteOrder};
use std::time::Duration;

#[derive(Debug, Clone, Copy, Default)]
pub struct GetSystemUpTime;

impl ShdlcCommand for GetSystemUpTime {
    type Response = u32;

    fn id(&self) -> u8 {
        0x93
    }

    fn data(&self) -> Vec<u8> {
        Vec::new()
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_millis(50)
    }

    fn min_response_length(&self) -> usize {
        4
    }

    fn max_response_length(&self) -> usize {
        4
    }

    fn interpret_response(&self, data: &[u8]) -> Result<Self::Response, ShdlcError> {
        Ok(BigEndian::read_u32(data))
    }
}
