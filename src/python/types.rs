use crate::protocol::types as proto_types;
use pyo3::prelude::*;

#[pyclass(name = "FirmwareVersion")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PyFirmwareVersion {
    #[pyo3(get, set)]
    pub major: u8,
    #[pyo3(get, set)]
    pub minor: u8,
    #[pyo3(get, set)]
    pub debug: bool,
}

#[pymethods]
impl PyFirmwareVersion {
    #[new]
    #[pyo3(signature = (major, minor, debug=false))]
    pub fn new(major: u8, minor: u8, debug: bool) -> Self {
        Self {
            major,
            minor,
            debug,
        }
    }

    pub fn __str__(&self) -> String {
        let v = proto_types::FirmwareVersion::new(self.major, self.minor, self.debug);
        v.to_string()
    }

    pub fn __repr__(&self) -> String {
        format!(
            "FirmwareVersion(major={}, minor={}, debug={})",
            self.major, self.minor, self.debug
        )
    }

    pub fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
}

impl From<proto_types::FirmwareVersion> for PyFirmwareVersion {
    fn from(v: proto_types::FirmwareVersion) -> Self {
        Self {
            major: v.major,
            minor: v.minor,
            debug: v.debug,
        }
    }
}

#[pyclass(name = "HardwareVersion")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PyHardwareVersion {
    #[pyo3(get, set)]
    pub major: u8,
    #[pyo3(get, set)]
    pub minor: u8,
}

#[pymethods]
impl PyHardwareVersion {
    #[new]
    pub fn new(major: u8, minor: u8) -> Self {
        Self { major, minor }
    }

    pub fn __str__(&self) -> String {
        let v = proto_types::HardwareVersion::new(self.major, self.minor);
        v.to_string()
    }

    pub fn __repr__(&self) -> String {
        format!(
            "HardwareVersion(major={}, minor={})",
            self.major, self.minor
        )
    }

    pub fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
}

impl From<proto_types::HardwareVersion> for PyHardwareVersion {
    fn from(v: proto_types::HardwareVersion) -> Self {
        Self {
            major: v.major,
            minor: v.minor,
        }
    }
}

#[pyclass(name = "ProtocolVersion")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PyProtocolVersion {
    #[pyo3(get, set)]
    pub major: u8,
    #[pyo3(get, set)]
    pub minor: u8,
}

#[pymethods]
impl PyProtocolVersion {
    #[new]
    pub fn new(major: u8, minor: u8) -> Self {
        Self { major, minor }
    }

    pub fn __str__(&self) -> String {
        let v = proto_types::ProtocolVersion::new(self.major, self.minor);
        v.to_string()
    }

    pub fn __repr__(&self) -> String {
        format!(
            "ProtocolVersion(major={}, minor={})",
            self.major, self.minor
        )
    }

    pub fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
}

impl From<proto_types::ProtocolVersion> for PyProtocolVersion {
    fn from(v: proto_types::ProtocolVersion) -> Self {
        Self {
            major: v.major,
            minor: v.minor,
        }
    }
}

#[pyclass(name = "Version")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PyVersion {
    #[pyo3(get, set)]
    pub firmware: PyFirmwareVersion,
    #[pyo3(get, set)]
    pub hardware: PyHardwareVersion,
    #[pyo3(get, set)]
    pub protocol: PyProtocolVersion,
}

#[pymethods]
impl PyVersion {
    #[new]
    pub fn new(
        firmware: PyFirmwareVersion,
        hardware: PyHardwareVersion,
        protocol: PyProtocolVersion,
    ) -> Self {
        Self {
            firmware,
            hardware,
            protocol,
        }
    }

    pub fn __str__(&self) -> String {
        format!(
            "Firmware {}, Hardware {}, Protocol {}",
            self.firmware.__str__(),
            self.hardware.__str__(),
            self.protocol.__str__()
        )
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Version(firmware={}, hardware={}, protocol={})",
            self.firmware.__repr__(),
            self.hardware.__repr__(),
            self.protocol.__repr__()
        )
    }

    pub fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
}

impl From<proto_types::Version> for PyVersion {
    fn from(v: proto_types::Version) -> Self {
        Self {
            firmware: v.firmware.into(),
            hardware: v.hardware.into(),
            protocol: v.protocol.into(),
        }
    }
}
