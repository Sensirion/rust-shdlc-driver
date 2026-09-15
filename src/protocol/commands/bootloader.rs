use crate::protocol::commands::ShdlcCommand;
use crate::protocol::errors::ShdlcError;
use std::time::Duration;

#[derive(Debug, Clone, Copy, Default)]
pub struct EnterBootloader;

impl ShdlcCommand for EnterBootloader {
    type Response = ();

    fn id(&self) -> u8 {
        0xF3
    }

    fn data(&self) -> Vec<u8> {
        Vec::new()
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_millis(100)
    }

    fn max_response_length(&self) -> usize {
        0
    }

    fn post_processing_time(&self) -> Duration {
        Duration::from_secs(2)
    }

    fn interpret_response(&self, _data: &[u8]) -> Result<Self::Response, ShdlcError> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FirmwareUpdateStart;

impl ShdlcCommand for FirmwareUpdateStart {
    type Response = ();

    fn id(&self) -> u8 {
        0xF3
    }

    fn data(&self) -> Vec<u8> {
        vec![0x01]
    }

    // Clearing flash can take long, allow 20 seconds
    fn max_response_time(&self) -> Duration {
        Duration::from_secs(20)
    }

    fn max_response_length(&self) -> usize {
        0
    }

    fn interpret_response(&self, _data: &[u8]) -> Result<Self::Response, ShdlcError> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct FirmwareUpdateData {
    pub payload: Vec<u8>,
}

impl FirmwareUpdateData {
    pub fn new(payload: &[u8]) -> Self {
        Self {
            payload: payload.to_vec(),
        }
    }
}

impl ShdlcCommand for FirmwareUpdateData {
    type Response = ();

    fn id(&self) -> u8 {
        0xF3
    }

    fn data(&self) -> Vec<u8> {
        let mut d = Vec::with_capacity(1 + self.payload.len());
        d.push(0x02);
        d.extend_from_slice(&self.payload);
        d
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_secs(1)
    }

    fn max_response_length(&self) -> usize {
        0
    }

    fn interpret_response(&self, _data: &[u8]) -> Result<Self::Response, ShdlcError> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FirmwareUpdateStop {
    pub checksum: u8,
}

impl FirmwareUpdateStop {
    pub fn new(checksum: u8) -> Self {
        Self { checksum }
    }
}

impl ShdlcCommand for FirmwareUpdateStop {
    type Response = ();

    fn id(&self) -> u8 {
        0xF3
    }

    fn data(&self) -> Vec<u8> {
        vec![0x03, self.checksum]
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_secs(1)
    }

    fn max_response_length(&self) -> usize {
        0
    }

    fn post_processing_time(&self) -> Duration {
        Duration::from_secs(2)
    }

    fn interpret_response(&self, _data: &[u8]) -> Result<Self::Response, ShdlcError> {
        Ok(())
    }
}
