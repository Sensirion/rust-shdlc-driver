use super::checksum::calculate_checksum;
use super::errors::ShdlcError;

pub const START_STOP_BYTE: u8 = 0x7E;
pub const ESCAPE_BYTE: u8 = 0x7D;
pub const ESCAPE_XOR: u8 = 0x20;
pub const CHARS_TO_ESCAPE: [u8; 4] = [START_STOP_BYTE, ESCAPE_BYTE, 0x11, 0x13];
pub const MAX_RAW_FRAME_LENGTH: usize = 522;

/// Perform SHDLC byte-stuffing on a raw byte slice.
pub fn stuff_data_bytes(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len() + 16);
    for &b in data {
        if CHARS_TO_ESCAPE.contains(&b) {
            result.push(ESCAPE_BYTE);
            result.push(b ^ ESCAPE_XOR);
        } else {
            result.push(b);
        }
    }
    result
}

/// Undo SHDLC byte-stuffing on a stuffed byte slice.
pub fn unstuff_data_bytes(stuffed_data: &[u8]) -> Vec<u8> {
    let mut data = Vec::with_capacity(stuffed_data.len());
    let mut xor = 0x00u8;
    for &b in stuffed_data {
        if b == ESCAPE_BYTE {
            xor = ESCAPE_XOR;
        } else {
            data.push(b ^ xor);
            xor = 0x00;
        }
    }
    data
}

/// MOSI (Master Out, Slave In) frame builder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShdlcMosiFrame {
    pub slave_address: u8,
    pub command_id: u8,
    pub data: Vec<u8>,
}

impl ShdlcMosiFrame {
    pub fn new(slave_address: u8, command_id: u8, data: &[u8]) -> Self {
        Self {
            slave_address,
            command_id,
            data: data.to_vec(),
        }
    }

    /// Encode the MOSI frame to raw SHDLC bytes ready to transmit on the wire.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut frame_content = Vec::with_capacity(4 + self.data.len());
        frame_content.push(self.slave_address);
        frame_content.push(self.command_id);
        frame_content.push(self.data.len() as u8);
        frame_content.extend_from_slice(&self.data);
        let checksum = calculate_checksum(&frame_content);
        frame_content.push(checksum);

        let stuffed = stuff_data_bytes(&frame_content);
        let mut raw_frame = Vec::with_capacity(stuffed.len() + 2);
        raw_frame.push(START_STOP_BYTE);
        raw_frame.extend_from_slice(&stuffed);
        raw_frame.push(START_STOP_BYTE);
        raw_frame
    }
}

/// MISO (Master In, Slave Out) decoded response frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShdlcMisoFrame {
    pub slave_address: u8,
    pub command_id: u8,
    pub state: u8,
    pub data: Vec<u8>,
}

impl ShdlcMisoFrame {
    pub fn is_error_state(&self) -> bool {
        (self.state & 0x80) != 0
    }

    pub fn error_code(&self) -> u8 {
        self.state & 0x7F
    }
}

/// MISO Frame Builder / Parser.
#[derive(Debug, Default, Clone)]
pub struct ShdlcMisoFrameBuilder {
    buffer: Vec<u8>,
}

impl ShdlcMisoFrameBuilder {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn data(&self) -> &[u8] {
        &self.buffer
    }

    pub fn start_received(&self) -> bool {
        self.buffer.contains(&START_STOP_BYTE)
    }

    /// Add incoming bytes. Returns Ok(true) if a full frame is received.
    pub fn add_data(&mut self, chunk: &[u8]) -> Result<bool, ShdlcError> {
        self.buffer.extend_from_slice(chunk);

        let start_count = self
            .buffer
            .iter()
            .filter(|&&b| b == START_STOP_BYTE)
            .count();
        if start_count >= 2 {
            Ok(true)
        } else if self.buffer.len() > MAX_RAW_FRAME_LENGTH {
            Err(ShdlcError::response_error(
                "Response is too long.",
                Some(self.buffer.clone()),
            ))
        } else {
            Ok(false)
        }
    }

    /// Interpret and validate received raw data into a `ShdlcMisoFrame`.
    pub fn interpret_data(&self) -> Result<ShdlcMisoFrame, ShdlcError> {
        // Find first START_STOP_BYTE and second START_STOP_BYTE
        let mut start_idx = None;
        let mut stop_idx = None;

        for (i, &b) in self.buffer.iter().enumerate() {
            if b == START_STOP_BYTE {
                if start_idx.is_none() {
                    start_idx = Some(i);
                } else if stop_idx.is_none() {
                    stop_idx = Some(i);
                    break;
                }
            }
        }

        let (start, stop) = match (start_idx, stop_idx) {
            (Some(s), Some(e)) if e > s => (s, e),
            _ => {
                return Err(ShdlcError::response_error(
                    "Response is too short.",
                    Some(self.buffer.clone()),
                ));
            }
        };

        let stuffed = &self.buffer[start + 1..stop];
        let unstuffed = unstuff_data_bytes(stuffed);

        if unstuffed.len() < 5 {
            return Err(ShdlcError::response_error(
                "Response is too short.",
                Some(self.buffer.clone()),
            ));
        }

        let frame = &unstuffed[..unstuffed.len() - 1];
        let address = frame[0];
        let command_id = frame[1];
        let state = frame[2];
        let length = frame[3] as usize;
        let data = frame[4..].to_vec();
        let checksum = unstuffed[unstuffed.len() - 1];

        if length != data.len() {
            return Err(ShdlcError::response_error(
                "Wrong length.",
                Some(self.buffer.clone()),
            ));
        }

        let calculated = calculate_checksum(frame);
        if checksum != calculated {
            return Err(ShdlcError::response_error(
                "Wrong checksum.",
                Some(self.buffer.clone()),
            ));
        }

        Ok(ShdlcMisoFrame {
            slave_address: address,
            command_id,
            state,
            data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mosi_to_bytes() {
        let frame = ShdlcMosiFrame::new(0x00, 0x00, &[]);
        assert_eq!(frame.to_bytes(), b"\x7e\x00\x00\x00\xff\x7e");

        let frame_stuffed = ShdlcMosiFrame::new(0x7e, 0x7d, &[0x11, 0x12, 0x13, 0x14]);
        assert_eq!(
            frame_stuffed.to_bytes(),
            b"\x7e\x7d\x5e\x7d\x5d\x04\x7d\x31\x12\x7d\x33\x14\xb6\x7e"
        );
    }

    #[test]
    fn test_miso_builder_valid() {
        let mut builder = ShdlcMisoFrameBuilder::new();
        assert!(!builder.start_received());
        let res = builder.add_data(b"\x7e\x00\x00\x00\x00\xff\x7e").unwrap();
        assert!(res);
        assert!(builder.start_received());

        let frame = builder.interpret_data().unwrap();
        assert_eq!(frame.slave_address, 0x00);
        assert_eq!(frame.command_id, 0x00);
        assert_eq!(frame.state, 0x00);
        assert_eq!(frame.data, Vec::<u8>::new());
    }

    #[test]
    fn test_miso_builder_byte_stuffing() {
        let mut builder = ShdlcMisoFrameBuilder::new();
        let raw = b"\x7e\x7d\x5e\x7d\x5d\x7d\x31\x03\x12\x7d\x33\x14\xb7\x7e";
        builder.add_data(raw).unwrap();
        let frame = builder.interpret_data().unwrap();
        assert_eq!(frame.slave_address, 0x7E);
        assert_eq!(frame.command_id, 0x7D);
        assert_eq!(frame.state, 0x11);
        assert_eq!(frame.data, vec![0x12, 0x13, 0x14]);
    }

    #[test]
    fn test_miso_builder_invalid_checksum() {
        let mut builder = ShdlcMisoFrameBuilder::new();
        builder.add_data(b"\x7e\x00\x00\x00\x00\xfe\x7e").unwrap();
        let err = builder.interpret_data().unwrap_err();
        match err {
            ShdlcError::ResponseError { message, .. } => {
                assert!(message.contains("Wrong checksum"));
            }
            _ => panic!("Expected ResponseError"),
        }
    }
}
