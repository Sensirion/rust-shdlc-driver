use crate::protocol::commands::ShdlcCommand;
use crate::protocol::errors::ShdlcError;
use std::time::Duration;

#[derive(Debug, Clone, Copy, Default)]
pub struct FactoryReset;

impl ShdlcCommand for FactoryReset {
    type Response = ();

    fn id(&self) -> u8 {
        0x92
    }

    fn data(&self) -> Vec<u8> {
        Vec::new()
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_secs(2)
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
