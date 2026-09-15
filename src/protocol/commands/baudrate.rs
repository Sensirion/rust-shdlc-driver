use crate::protocol::commands::ShdlcCommand;
use crate::protocol::errors::ShdlcError;
use byteorder::{BigEndian, ByteOrder};
use std::time::Duration;

#[derive(Debug, Clone, Copy, Default)]
pub struct GetBaudrate;

impl ShdlcCommand for GetBaudrate {
    type Response = u32;

    fn id(&self) -> u8 {
        0x91
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

#[derive(Debug, Clone, Copy)]
pub struct SetBaudrate {
    pub baudrate: u32,
}

impl SetBaudrate {
    pub fn new(baudrate: u32) -> Self {
        Self { baudrate }
    }
}

impl ShdlcCommand for SetBaudrate {
    type Response = ();

    fn id(&self) -> u8 {
        0x91
    }

    fn data(&self) -> Vec<u8> {
        let mut buf = vec![0u8; 4];
        BigEndian::write_u32(&mut buf, self.baudrate);
        buf
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_millis(50)
    }

    fn max_response_length(&self) -> usize {
        0
    }

    fn interpret_response(&self, _data: &[u8]) -> Result<Self::Response, ShdlcError> {
        Ok(())
    }
}
