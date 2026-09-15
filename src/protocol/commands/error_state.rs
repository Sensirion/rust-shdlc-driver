use crate::protocol::commands::ShdlcCommand;
use crate::protocol::errors::ShdlcError;
use byteorder::{BigEndian, ByteOrder};
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct GetErrorState {
    pub clear: bool,
}

impl GetErrorState {
    pub fn new(clear: bool) -> Self {
        Self { clear }
    }
}

impl Default for GetErrorState {
    fn default() -> Self {
        Self { clear: true }
    }
}

impl ShdlcCommand for GetErrorState {
    type Response = (u32, u8);

    fn id(&self) -> u8 {
        0xD2
    }

    fn data(&self) -> Vec<u8> {
        vec![if self.clear { 0x01 } else { 0x00 }]
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_millis(500)
    }

    fn min_response_length(&self) -> usize {
        5
    }

    fn max_response_length(&self) -> usize {
        5
    }

    fn interpret_response(&self, data: &[u8]) -> Result<Self::Response, ShdlcError> {
        let device_state = BigEndian::read_u32(&data[0..4]);
        let last_error = data[4];
        Ok((device_state, last_error))
    }
}
