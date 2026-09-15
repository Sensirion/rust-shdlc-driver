use crate::protocol::commands::ShdlcCommand;
use crate::protocol::errors::ShdlcError;
use std::time::Duration;

fn decode_null_terminated_string(data: &[u8]) -> Result<String, ShdlcError> {
    let s = String::from_utf8_lossy(data);
    Ok(s.trim_end_matches('\0').to_string())
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GetProductType;

impl ShdlcCommand for GetProductType {
    type Response = String;

    fn id(&self) -> u8 {
        0xD0
    }

    fn data(&self) -> Vec<u8> {
        vec![0x00]
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_millis(500)
    }

    fn interpret_response(&self, data: &[u8]) -> Result<Self::Response, ShdlcError> {
        decode_null_terminated_string(data)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GetProductName;

impl ShdlcCommand for GetProductName {
    type Response = String;

    fn id(&self) -> u8 {
        0xD0
    }

    fn data(&self) -> Vec<u8> {
        vec![0x01]
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_millis(500)
    }

    fn interpret_response(&self, data: &[u8]) -> Result<Self::Response, ShdlcError> {
        decode_null_terminated_string(data)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GetArticleCode;

impl ShdlcCommand for GetArticleCode {
    type Response = String;

    fn id(&self) -> u8 {
        0xD0
    }

    fn data(&self) -> Vec<u8> {
        vec![0x02]
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_millis(500)
    }

    fn interpret_response(&self, data: &[u8]) -> Result<Self::Response, ShdlcError> {
        decode_null_terminated_string(data)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GetSerialNumber;

impl ShdlcCommand for GetSerialNumber {
    type Response = String;

    fn id(&self) -> u8 {
        0xD0
    }

    fn data(&self) -> Vec<u8> {
        vec![0x03]
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_millis(500)
    }

    fn interpret_response(&self, data: &[u8]) -> Result<Self::Response, ShdlcError> {
        decode_null_terminated_string(data)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GetProductSubType;

impl ShdlcCommand for GetProductSubType {
    type Response = u8;

    fn id(&self) -> u8 {
        0xD0
    }

    fn data(&self) -> Vec<u8> {
        vec![0x04]
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_millis(500)
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
