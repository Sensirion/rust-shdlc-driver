use thiserror::Error;

/// SHDLC error enum covering all protocol, transport, device, and firmware update errors.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ShdlcError {
    #[error("Timeout while waiting for response from SHDLC device. Check connection to device and make sure it is powered on.")]
    Timeout,

    #[error("Invalid data received from the SHDLC device: {message}")]
    ResponseError {
        message: String,
        raw_data: Option<Vec<u8>>,
    },

    #[error("SHDLC device returned error code {code}: {message}")]
    DeviceError { code: u8, message: String },

    #[error("Invalid signature in firmware image: 0x{}", hex_encode(.0))]
    FirmwareImageSignatureError(Vec<u8>),

    #[error("Firmware image for device 0x{expected:08X} not compatible with connected device 0x{actual:08X}.")]
    FirmwareImageIncompatibilityError { expected: u32, actual: u32 },

    #[error("Transport error: {0}")]
    Transport(String),

    #[error("I/O error: {0}")]
    Io(String),

    #[error("Port error: {0}")]
    PortError(String),

    #[error("{0}")]
    Other(String),
}

fn hex_encode(data: &[u8]) -> String {
    data.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<String>()
}

impl ShdlcError {
    pub fn response_error(msg: impl Into<String>, raw_data: Option<Vec<u8>>) -> Self {
        ShdlcError::ResponseError {
            message: msg.into(),
            raw_data,
        }
    }

    pub fn device_error(code: u8) -> Self {
        let msg = match code {
            1 => "Illegal data size of the MOSI frame. Either a wrong command was sent, or the device firmware does not support the requested feature.",
            2 => "Unknown command. Check if you sent the correct command and if the firmware on the device supports it.",
            3 => "No access right for this command. Higher access rights are required to execute this command.",
            4 => "Parameter out of range. Check if you sent the correct command parameters and if the firmware on the device supports them.",
            5 => "Wrong checksum received.",
            6 => "Firmware update operation failed. Flash couldn't be written or flash validation failed.",
            _ => "Unknown error.",
        };
        ShdlcError::DeviceError {
            code,
            message: msg.to_string(),
        }
    }
}

impl From<std::io::Error> for ShdlcError {
    fn from(err: std::io::Error) -> Self {
        if err.kind() == std::io::ErrorKind::TimedOut {
            ShdlcError::Timeout
        } else {
            ShdlcError::Io(err.to_string())
        }
    }
}
