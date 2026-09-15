use crate::protocol::commands::ShdlcCommand;
use crate::protocol::errors::ShdlcError;
use crate::protocol::types::{FirmwareVersion, HardwareVersion, ProtocolVersion, Version};
use std::time::Duration;

#[derive(Debug, Clone, Copy, Default)]
pub struct GetVersion;

impl ShdlcCommand for GetVersion {
    type Response = Version;

    fn id(&self) -> u8 {
        0xD1
    }

    fn data(&self) -> Vec<u8> {
        Vec::new()
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_millis(500)
    }

    fn min_response_length(&self) -> usize {
        7
    }

    fn max_response_length(&self) -> usize {
        7
    }

    fn interpret_response(&self, data: &[u8]) -> Result<Self::Response, ShdlcError> {
        Ok(Version::new(
            FirmwareVersion::new(data[0], data[1], data[2] != 0),
            HardwareVersion::new(data[3], data[4]),
            ProtocolVersion::new(data[5], data[6]),
        ))
    }
}
