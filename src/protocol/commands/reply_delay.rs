use crate::protocol::commands::ShdlcCommand;
use crate::protocol::errors::ShdlcError;
use byteorder::{BigEndian, ByteOrder};
use std::time::Duration;

#[derive(Debug, Clone, Copy, Default)]
pub struct GetReplyDelay;

impl ShdlcCommand for GetReplyDelay {
    type Response = u16;

    fn id(&self) -> u8 {
        0x95
    }

    fn data(&self) -> Vec<u8> {
        Vec::new()
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_millis(50)
    }

    fn min_response_length(&self) -> usize {
        2
    }

    fn max_response_length(&self) -> usize {
        2
    }

    fn interpret_response(&self, data: &[u8]) -> Result<Self::Response, ShdlcError> {
        Ok(BigEndian::read_u16(data))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SetReplyDelay {
    pub reply_delay_us: u16,
}

impl SetReplyDelay {
    pub fn new(reply_delay_us: u16) -> Self {
        Self { reply_delay_us }
    }
}

impl ShdlcCommand for SetReplyDelay {
    type Response = ();

    fn id(&self) -> u8 {
        0x95
    }

    fn data(&self) -> Vec<u8> {
        let mut buf = vec![0u8; 2];
        BigEndian::write_u16(&mut buf, self.reply_delay_us);
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
