use crate::protocol::commands::ShdlcCommand;
use crate::protocol::errors::ShdlcError;
use std::time::Duration;

#[derive(Debug, Clone, Copy, Default)]
pub struct GetSlaveAddress;

impl ShdlcCommand for GetSlaveAddress {
    type Response = u8;

    fn id(&self) -> u8 {
        0x90
    }

    fn data(&self) -> Vec<u8> {
        Vec::new()
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_millis(50)
    }

    fn min_response_length(&self) -> usize {
        1
    }

    fn max_response_length(&self) -> usize {
        1
    }

    fn interpret_response(&self, data: &[u8]) -> Result<Self::Response, ShdlcError> {
        Ok(data[0])
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SetSlaveAddress {
    pub slave_address: u8,
}

impl SetSlaveAddress {
    pub fn new(slave_address: u8) -> Self {
        Self { slave_address }
    }
}

impl ShdlcCommand for SetSlaveAddress {
    type Response = ();

    fn id(&self) -> u8 {
        0x90
    }

    fn data(&self) -> Vec<u8> {
        vec![self.slave_address]
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
