use crate::protocol::errors::ShdlcError;
use std::time::Duration;

pub mod baudrate;
pub mod bootloader;
pub mod custom;
pub mod device_info;
pub mod device_reset;
pub mod device_version;
pub mod error_state;
pub mod factory_reset;
pub mod reply_delay;
pub mod slave_address;
pub mod system_up_time;

pub use baudrate::{GetBaudrate, SetBaudrate};
pub use bootloader::{
    EnterBootloader, FirmwareUpdateData, FirmwareUpdateStart, FirmwareUpdateStop,
};
pub use custom::RawShdlcCommand;
pub use device_info::{
    GetArticleCode, GetProductName, GetProductSubType, GetProductType, GetSerialNumber,
};
pub use device_reset::DeviceReset;
pub use device_version::GetVersion;
pub use error_state::GetErrorState;
pub use factory_reset::FactoryReset;
pub use reply_delay::{GetReplyDelay, SetReplyDelay};
pub use slave_address::{GetSlaveAddress, SetSlaveAddress};
pub use system_up_time::GetSystemUpTime;

/// Trait implemented by all SHDLC commands.
pub trait ShdlcCommand: Send + Sync {
    type Response: Send;

    fn id(&self) -> u8;
    fn data(&self) -> Vec<u8>;
    fn max_response_time(&self) -> Duration;

    fn min_response_length(&self) -> usize {
        0
    }

    fn max_response_length(&self) -> usize {
        255
    }

    fn post_processing_time(&self) -> Duration {
        Duration::from_secs(0)
    }

    fn check_response_length(&self, data: &[u8]) -> Result<(), ShdlcError> {
        let len = data.len();
        if len < self.min_response_length() || len > self.max_response_length() {
            Err(ShdlcError::response_error(
                format!(
                    "Wrong response length (expected {}..{} bytes, got {}).",
                    self.min_response_length(),
                    self.max_response_length(),
                    len
                ),
                Some(data.to_vec()),
            ))
        } else {
            Ok(())
        }
    }

    fn interpret_response(&self, data: &[u8]) -> Result<Self::Response, ShdlcError>;
}
