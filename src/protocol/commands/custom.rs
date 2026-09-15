use crate::protocol::commands::ShdlcCommand;
use crate::protocol::errors::ShdlcError;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RawShdlcCommand {
    pub id: u8,
    pub data: Vec<u8>,
    pub max_response_time: Duration,
    pub min_response_length: usize,
    pub max_response_length: usize,
    pub post_processing_time: Duration,
}

impl RawShdlcCommand {
    pub fn new(id: u8, data: &[u8], max_response_time: Duration) -> Self {
        Self {
            id,
            data: data.to_vec(),
            max_response_time,
            min_response_length: 0,
            max_response_length: 255,
            post_processing_time: Duration::from_secs(0),
        }
    }

    pub fn with_response_lengths(mut self, min: usize, max: usize) -> Self {
        self.min_response_length = min;
        self.max_response_length = max;
        self
    }

    pub fn with_post_processing_time(mut self, time: Duration) -> Self {
        self.post_processing_time = time;
        self
    }
}

impl ShdlcCommand for RawShdlcCommand {
    type Response = Vec<u8>;

    fn id(&self) -> u8 {
        self.id
    }

    fn data(&self) -> Vec<u8> {
        self.data.clone()
    }

    fn max_response_time(&self) -> Duration {
        self.max_response_time
    }

    fn min_response_length(&self) -> usize {
        self.min_response_length
    }

    fn max_response_length(&self) -> usize {
        self.max_response_length
    }

    fn post_processing_time(&self) -> Duration {
        self.post_processing_time
    }

    fn interpret_response(&self, data: &[u8]) -> Result<Self::Response, ShdlcError> {
        Ok(data.to_vec())
    }
}
