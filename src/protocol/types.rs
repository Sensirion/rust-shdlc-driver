use std::fmt;

/// Represents a firmware version of an SHDLC device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FirmwareVersion {
    pub major: u8,
    pub minor: u8,
    pub debug: bool,
}

impl FirmwareVersion {
    pub fn new(major: u8, minor: u8, debug: bool) -> Self {
        Self {
            major,
            minor,
            debug,
        }
    }
}

impl fmt::Display for FirmwareVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.debug {
            write!(f, "{}.{}-debug", self.major, self.minor)
        } else {
            write!(f, "{}.{}", self.major, self.minor)
        }
    }
}

/// Represents a hardware version of an SHDLC device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HardwareVersion {
    pub major: u8,
    pub minor: u8,
}

impl HardwareVersion {
    pub fn new(major: u8, minor: u8) -> Self {
        Self { major, minor }
    }
}

impl fmt::Display for HardwareVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

/// Represents a protocol version of an SHDLC device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProtocolVersion {
    pub major: u8,
    pub minor: u8,
}

impl ProtocolVersion {
    pub fn new(major: u8, minor: u8) -> Self {
        Self { major, minor }
    }
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

/// Represents all version numbers of an SHDLC device (Firmware, Hardware, Protocol).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Version {
    pub firmware: FirmwareVersion,
    pub hardware: HardwareVersion,
    pub protocol: ProtocolVersion,
}

impl Version {
    pub fn new(
        firmware: FirmwareVersion,
        hardware: HardwareVersion,
        protocol: ProtocolVersion,
    ) -> Self {
        Self {
            firmware,
            hardware,
            protocol,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Firmware {}, Hardware {}, Protocol {}",
            self.firmware, self.hardware, self.protocol
        )
    }
}
