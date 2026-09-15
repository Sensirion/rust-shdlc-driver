pub mod checksum;
pub mod commands;
pub mod errors;
pub mod frame;
pub mod types;

pub use checksum::calculate_checksum;
pub use commands::*;
pub use errors::ShdlcError;
pub use frame::{
    stuff_data_bytes, unstuff_data_bytes, ShdlcMisoFrame, ShdlcMisoFrameBuilder, ShdlcMosiFrame,
    CHARS_TO_ESCAPE, ESCAPE_BYTE, ESCAPE_XOR, MAX_RAW_FRAME_LENGTH, START_STOP_BYTE,
};
pub use types::{FirmwareVersion, HardwareVersion, ProtocolVersion, Version};
