use crate::protocol::errors::ShdlcError;
use std::collections::BTreeMap;

/// Simple, robust parser for Intel-Hex (.hex) files.
#[derive(Debug, Clone)]
pub struct IntelHexParser {
    memory: BTreeMap<u32, u8>,
    padding: u8,
}

impl Default for IntelHexParser {
    fn default() -> Self {
        Self::new()
    }
}

impl IntelHexParser {
    pub fn new() -> Self {
        Self {
            memory: BTreeMap::new(),
            padding: 0xFF,
        }
    }

    pub fn from_hex_str(hex_str: &str) -> Result<Self, ShdlcError> {
        let mut parser = Self::new();
        parser.load_hex_str(hex_str)?;
        Ok(parser)
    }

    pub fn load_hex_str(&mut self, hex_str: &str) -> Result<(), ShdlcError> {
        let mut base_address: u32 = 0;

        for (line_num, line) in hex_str.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if !line.starts_with(':') {
                return Err(ShdlcError::Other(format!(
                    "Invalid Intel-Hex line {} (missing ':' prefix)",
                    line_num + 1
                )));
            }

            let hex_payload = &line[1..];
            if hex_payload.len() < 10 {
                return Err(ShdlcError::Other(format!(
                    "Intel-Hex line {} too short",
                    line_num + 1
                )));
            }

            let bytes = hex::decode(hex_payload).map_err(|e| {
                ShdlcError::Other(format!(
                    "Invalid hex encoding on line {}: {}",
                    line_num + 1,
                    e
                ))
            })?;

            let byte_count = bytes[0] as usize;
            let offset = ((bytes[1] as u32) << 8) | (bytes[2] as u32);
            let record_type = bytes[3];
            let data = &bytes[4..4 + byte_count];
            let _checksum = bytes[bytes.len() - 1];

            // Verify checksum: sum of all bytes modulo 256 must be 0
            let sum: u8 = bytes.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
            if sum != 0 {
                return Err(ShdlcError::Other(format!(
                    "Intel-Hex checksum mismatch on line {}",
                    line_num + 1
                )));
            }

            match record_type {
                0x00 => {
                    // Data record
                    let start_addr = base_address + offset;
                    for (i, &b) in data.iter().enumerate() {
                        self.memory.insert(start_addr + i as u32, b);
                    }
                }
                0x01 => {
                    // End of File
                    break;
                }
                0x02 if data.len() >= 2 => {
                    // Extended Segment Address
                    let seg = ((data[0] as u32) << 8) | (data[1] as u32);
                    base_address = seg << 4;
                }
                0x04 if data.len() >= 2 => {
                    // Extended Linear Address
                    let high = ((data[0] as u32) << 8) | (data[1] as u32);
                    base_address = high << 16;
                }
                _ => {
                    // Ignore unsupported record types
                }
            }
        }

        Ok(())
    }

    pub fn min_addr(&self) -> u32 {
        self.memory.keys().next().copied().unwrap_or(0)
    }

    pub fn max_addr(&self) -> u32 {
        self.memory.keys().next_back().copied().unwrap_or(0)
    }

    pub fn read_bytes(&self, start: u32, size: usize) -> Vec<u8> {
        let mut out = Vec::with_capacity(size);
        for addr in start..start + size as u32 {
            out.push(*self.memory.get(&addr).unwrap_or(&self.padding));
        }
        out
    }

    pub fn read_range(&self, start: u32, end_inclusive: u32) -> Vec<u8> {
        if end_inclusive < start {
            return Vec::new();
        }
        let size = (end_inclusive - start + 1) as usize;
        self.read_bytes(start, size)
    }

    pub fn read_u32_le(&self, start: u32) -> u32 {
        let bytes = self.read_bytes(start, 4);
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

// Simple hex helper without extra external crate
mod hex {
    pub fn decode(s: &str) -> Result<Vec<u8>, &'static str> {
        if !s.len().is_multiple_of(2) {
            return Err("Odd length");
        }
        let mut bytes = Vec::with_capacity(s.len() / 2);
        for i in (0..s.len()).step_by(2) {
            let byte = u8::from_str_radix(&s[i..i + 2], 16).map_err(|_| "Invalid hex char")?;
            bytes.push(byte);
        }
        Ok(bytes)
    }
}
