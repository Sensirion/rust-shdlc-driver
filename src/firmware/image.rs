use crate::firmware::hex_parser::IntelHexParser;
use crate::protocol::errors::ShdlcError;
use crate::protocol::types::FirmwareVersion;

const PRODUCT_TYPE_SIZE: u32 = 4;

/// Represents a loaded and validated firmware image for an SHDLC device.
#[derive(Debug, Clone)]
pub struct ShdlcFirmwareImage {
    _bl_start_addr: u32,
    _app_start_addr: u32,
    _signature: Vec<u8>,
    _bl_version_offset: u32,
    app_data_index: usize,
    product_type: u32,
    bootloader_version: FirmwareVersion,
    application_version: FirmwareVersion,
    app_data: Vec<u8>,
    checksum: u8,
}

impl ShdlcFirmwareImage {
    pub fn new(
        hex_content: &str,
        bl_start_addr: u32,
        app_start_addr: u32,
        signature: Option<&[u8]>,
        bl_version_offset: Option<u32>,
    ) -> Result<Self, ShdlcError> {
        let sig = signature.unwrap_or(b"\x4A\x47\x4F\x4B").to_vec();
        let bl_ver_off = bl_version_offset.unwrap_or(0x1004);

        let hex = IntelHexParser::from_hex_str(hex_content)?;

        // Check signature
        let actual_sig = hex.read_bytes(app_start_addr, sig.len());
        if actual_sig != sig {
            return Err(ShdlcError::FirmwareImageSignatureError(actual_sig));
        }

        // Read product type
        let pt_addr = app_start_addr + sig.len() as u32;
        let product_type = hex.read_u32_le(pt_addr);

        // Read bootloader version
        let bl_ver_minor_addr = bl_start_addr + bl_ver_off;
        let bl_ver_bytes = hex.read_bytes(bl_ver_minor_addr, 2);
        let bootloader_version = FirmwareVersion::new(bl_ver_bytes[1], bl_ver_bytes[0], false);

        // Read application version
        let app_ver_minor_addr = app_start_addr + sig.len() as u32 + PRODUCT_TYPE_SIZE;
        let app_ver_bytes = hex.read_bytes(app_ver_minor_addr, 2);
        let application_version = FirmwareVersion::new(app_ver_bytes[1], app_ver_bytes[0], false);

        // Read application data
        let start_addr = app_start_addr + sig.len() as u32;
        let end_addr = if bl_start_addr > app_start_addr {
            bl_start_addr - 1
        } else {
            hex.max_addr()
        };

        let app_data = hex.read_range(start_addr, end_addr);
        let sum: u32 = app_data.iter().map(|&b| b as u32).sum();
        let checksum = ((sum % 256) as u8) ^ 0xFF;

        Ok(Self {
            _bl_start_addr: bl_start_addr,
            _app_start_addr: app_start_addr,
            _signature: sig,
            _bl_version_offset: bl_ver_off,
            app_data_index: 0,
            product_type,
            bootloader_version,
            application_version,
            app_data,
            checksum,
        })
    }

    pub fn product_type(&self) -> u32 {
        self.product_type
    }

    pub fn bootloader_version(&self) -> FirmwareVersion {
        self.bootloader_version
    }

    pub fn application_version(&self) -> FirmwareVersion {
        self.application_version
    }

    pub fn checksum(&self) -> u8 {
        self.checksum
    }

    pub fn size(&self) -> usize {
        self.app_data.len()
    }

    pub fn available_bytes(&self) -> usize {
        self.app_data.len().saturating_sub(self.app_data_index)
    }

    pub fn reset_cursor(&mut self) {
        self.app_data_index = 0;
    }

    pub fn read(&mut self, size: Option<usize>) -> Vec<u8> {
        let avail = self.available_bytes();
        let count = match size {
            Some(s) => std::cmp::min(s, avail),
            None => avail,
        };

        let chunk = self.app_data[self.app_data_index..self.app_data_index + count].to_vec();
        self.app_data_index += count;
        chunk
    }
}
