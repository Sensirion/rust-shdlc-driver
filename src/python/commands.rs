use crate::python::errors::to_py_err;
use crate::python::types::PyVersion;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyclass(name = "ShdlcCommand", subclass)]
#[derive(Debug, Clone)]
pub struct PyShdlcCommand {
    #[pyo3(get, set)]
    pub id: u8,
    #[pyo3(get, set)]
    pub data: Vec<u8>,
    #[pyo3(get, set)]
    pub max_response_time: f64,
    #[pyo3(get, set)]
    pub min_response_length: usize,
    #[pyo3(get, set)]
    pub max_response_length: usize,
    #[pyo3(get, set)]
    pub post_processing_time: f64,
}

#[pymethods]
impl PyShdlcCommand {
    #[new]
    #[pyo3(signature = (id, data, max_response_time, min_response_length=0, max_response_length=255, post_processing_time=0.0))]
    pub fn new(
        id: u8,
        data: Vec<u8>,
        max_response_time: f64,
        min_response_length: usize,
        max_response_length: usize,
        post_processing_time: f64,
    ) -> Self {
        Self {
            id,
            data,
            max_response_time,
            min_response_length,
            max_response_length,
            post_processing_time,
        }
    }

    pub fn check_response_length(&self, py: Python<'_>, data: &[u8]) -> PyResult<()> {
        let len = data.len();
        if len < self.min_response_length || len > self.max_response_length {
            let err = crate::protocol::errors::ShdlcError::response_error(
                format!(
                    "Wrong response length (expected {}..{} bytes, got {}).",
                    self.min_response_length, self.max_response_length, len
                ),
                Some(data.to_vec()),
            );
            Err(to_py_err(py, err))
        } else {
            Ok(())
        }
    }

    pub fn interpret_response<'py>(
        &self,
        py: Python<'py>,
        data: &[u8],
    ) -> PyResult<Option<Bound<'py, PyBytes>>> {
        if data.is_empty() {
            Ok(None)
        } else {
            Ok(Some(PyBytes::new(py, data)))
        }
    }
}

// Builtin command factories matching Python API:
#[pyclass(name = "ShdlcCmdGetProductType", extends = PyShdlcCommand)]
pub struct PyShdlcCmdGetProductType;

#[pymethods]
impl PyShdlcCmdGetProductType {
    #[new]
    pub fn new() -> (Self, PyShdlcCommand) {
        (
            PyShdlcCmdGetProductType,
            PyShdlcCommand::new(0xD0, vec![0x00], 0.5, 0, 255, 0.0),
        )
    }

    pub fn interpret_response(&self, data: &[u8]) -> PyResult<String> {
        let s = String::from_utf8_lossy(data);
        Ok(s.trim_end_matches('\0').to_string())
    }
}

#[pyclass(name = "ShdlcCmdGetProductName", extends = PyShdlcCommand)]
pub struct PyShdlcCmdGetProductName;

#[pymethods]
impl PyShdlcCmdGetProductName {
    #[new]
    pub fn new() -> (Self, PyShdlcCommand) {
        (
            PyShdlcCmdGetProductName,
            PyShdlcCommand::new(0xD0, vec![0x01], 0.5, 0, 255, 0.0),
        )
    }

    pub fn interpret_response(&self, data: &[u8]) -> PyResult<String> {
        let s = String::from_utf8_lossy(data);
        Ok(s.trim_end_matches('\0').to_string())
    }
}

#[pyclass(name = "ShdlcCmdGetArticleCode", extends = PyShdlcCommand)]
pub struct PyShdlcCmdGetArticleCode;

#[pymethods]
impl PyShdlcCmdGetArticleCode {
    #[new]
    pub fn new() -> (Self, PyShdlcCommand) {
        (
            PyShdlcCmdGetArticleCode,
            PyShdlcCommand::new(0xD0, vec![0x02], 0.5, 0, 255, 0.0),
        )
    }

    pub fn interpret_response(&self, data: &[u8]) -> PyResult<String> {
        let s = String::from_utf8_lossy(data);
        Ok(s.trim_end_matches('\0').to_string())
    }
}

#[pyclass(name = "ShdlcCmdGetSerialNumber", extends = PyShdlcCommand)]
pub struct PyShdlcCmdGetSerialNumber;

#[pymethods]
impl PyShdlcCmdGetSerialNumber {
    #[new]
    pub fn new() -> (Self, PyShdlcCommand) {
        (
            PyShdlcCmdGetSerialNumber,
            PyShdlcCommand::new(0xD0, vec![0x03], 0.5, 0, 255, 0.0),
        )
    }

    pub fn interpret_response(&self, data: &[u8]) -> PyResult<String> {
        let s = String::from_utf8_lossy(data);
        Ok(s.trim_end_matches('\0').to_string())
    }
}

#[pyclass(name = "ShdlcCmdGetProductSubType", extends = PyShdlcCommand)]
pub struct PyShdlcCmdGetProductSubType;

#[pymethods]
impl PyShdlcCmdGetProductSubType {
    #[new]
    pub fn new() -> (Self, PyShdlcCommand) {
        (
            PyShdlcCmdGetProductSubType,
            PyShdlcCommand::new(0xD0, vec![0x04], 0.5, 1, 1, 0.0),
        )
    }

    pub fn interpret_response(&self, data: &[u8]) -> PyResult<u8> {
        Ok(data[0])
    }
}

#[pyclass(name = "ShdlcCmdGetVersion", extends = PyShdlcCommand)]
pub struct PyShdlcCmdGetVersion;

#[pymethods]
impl PyShdlcCmdGetVersion {
    #[new]
    pub fn new() -> (Self, PyShdlcCommand) {
        (
            PyShdlcCmdGetVersion,
            PyShdlcCommand::new(0xD1, vec![], 0.5, 7, 7, 0.0),
        )
    }

    pub fn interpret_response(&self, data: &[u8]) -> PyResult<PyVersion> {
        Ok(PyVersion::new(
            crate::python::types::PyFirmwareVersion::new(data[0], data[1], data[2] != 0),
            crate::python::types::PyHardwareVersion::new(data[3], data[4]),
            crate::python::types::PyProtocolVersion::new(data[5], data[6]),
        ))
    }
}

#[pyclass(name = "ShdlcCmdGetErrorState", extends = PyShdlcCommand)]
pub struct PyShdlcCmdGetErrorState;

#[pymethods]
impl PyShdlcCmdGetErrorState {
    #[new]
    #[pyo3(signature = (clear=true))]
    pub fn new(clear: bool) -> (Self, PyShdlcCommand) {
        (
            PyShdlcCmdGetErrorState,
            PyShdlcCommand::new(0xD2, vec![if clear { 0x01 } else { 0x00 }], 0.5, 5, 5, 0.0),
        )
    }

    pub fn interpret_response(&self, data: &[u8]) -> PyResult<(u32, u8)> {
        let state = byteorder::BigEndian::read_u32(&data[0..4]);
        let last_error = data[4];
        Ok((state, last_error))
    }
}

#[pyclass(name = "ShdlcCmdDeviceReset", extends = PyShdlcCommand)]
pub struct PyShdlcCmdDeviceReset;

#[pymethods]
impl PyShdlcCmdDeviceReset {
    #[new]
    pub fn new() -> (Self, PyShdlcCommand) {
        (
            PyShdlcCmdDeviceReset,
            PyShdlcCommand::new(0xD3, vec![], 0.5, 0, 0, 2.0),
        )
    }
}

#[pyclass(name = "ShdlcCmdGetSlaveAddress", extends = PyShdlcCommand)]
pub struct PyShdlcCmdGetSlaveAddress;

#[pymethods]
impl PyShdlcCmdGetSlaveAddress {
    #[new]
    pub fn new() -> (Self, PyShdlcCommand) {
        (
            PyShdlcCmdGetSlaveAddress,
            PyShdlcCommand::new(0x90, vec![], 0.05, 1, 1, 0.0),
        )
    }

    pub fn interpret_response(&self, data: &[u8]) -> PyResult<u8> {
        Ok(data[0])
    }
}

#[pyclass(name = "ShdlcCmdSetSlaveAddress", extends = PyShdlcCommand)]
pub struct PyShdlcCmdSetSlaveAddress;

#[pymethods]
impl PyShdlcCmdSetSlaveAddress {
    #[new]
    pub fn new(slave_address: u8) -> (Self, PyShdlcCommand) {
        (
            PyShdlcCmdSetSlaveAddress,
            PyShdlcCommand::new(0x90, vec![slave_address], 0.05, 0, 0, 0.0),
        )
    }
}

#[pyclass(name = "ShdlcCmdGetBaudrate", extends = PyShdlcCommand)]
pub struct PyShdlcCmdGetBaudrate;

#[pymethods]
impl PyShdlcCmdGetBaudrate {
    #[new]
    pub fn new() -> (Self, PyShdlcCommand) {
        (
            PyShdlcCmdGetBaudrate,
            PyShdlcCommand::new(0x91, vec![], 0.05, 4, 4, 0.0),
        )
    }

    pub fn interpret_response(&self, data: &[u8]) -> PyResult<u32> {
        Ok(byteorder::BigEndian::read_u32(data))
    }
}

#[pyclass(name = "ShdlcCmdSetBaudrate", extends = PyShdlcCommand)]
pub struct PyShdlcCmdSetBaudrate;

#[pymethods]
impl PyShdlcCmdSetBaudrate {
    #[new]
    pub fn new(baudrate: u32) -> (Self, PyShdlcCommand) {
        let mut buf = vec![0u8; 4];
        byteorder::BigEndian::write_u32(&mut buf, baudrate);
        (
            PyShdlcCmdSetBaudrate,
            PyShdlcCommand::new(0x91, buf, 0.05, 0, 0, 0.0),
        )
    }
}

#[pyclass(name = "ShdlcCmdGetReplyDelay", extends = PyShdlcCommand)]
pub struct PyShdlcCmdGetReplyDelay;

#[pymethods]
impl PyShdlcCmdGetReplyDelay {
    #[new]
    pub fn new() -> (Self, PyShdlcCommand) {
        (
            PyShdlcCmdGetReplyDelay,
            PyShdlcCommand::new(0x95, vec![], 0.05, 2, 2, 0.0),
        )
    }

    pub fn interpret_response(&self, data: &[u8]) -> PyResult<u16> {
        Ok(byteorder::BigEndian::read_u16(data))
    }
}

#[pyclass(name = "ShdlcCmdSetReplyDelay", extends = PyShdlcCommand)]
pub struct PyShdlcCmdSetReplyDelay;

#[pymethods]
impl PyShdlcCmdSetReplyDelay {
    #[new]
    pub fn new(reply_delay: u16) -> (Self, PyShdlcCommand) {
        let mut buf = vec![0u8; 2];
        byteorder::BigEndian::write_u16(&mut buf, reply_delay);
        (
            PyShdlcCmdSetReplyDelay,
            PyShdlcCommand::new(0x95, buf, 0.05, 0, 0, 0.0),
        )
    }
}

#[pyclass(name = "ShdlcCmdGetSystemUpTime", extends = PyShdlcCommand)]
pub struct PyShdlcCmdGetSystemUpTime;

#[pymethods]
impl PyShdlcCmdGetSystemUpTime {
    #[new]
    pub fn new() -> (Self, PyShdlcCommand) {
        (
            PyShdlcCmdGetSystemUpTime,
            PyShdlcCommand::new(0x93, vec![], 0.05, 4, 4, 0.0),
        )
    }

    pub fn interpret_response(&self, data: &[u8]) -> PyResult<u32> {
        Ok(byteorder::BigEndian::read_u32(data))
    }
}

#[pyclass(name = "ShdlcCmdFactoryReset", extends = PyShdlcCommand)]
pub struct PyShdlcCmdFactoryReset;

#[pymethods]
impl PyShdlcCmdFactoryReset {
    #[new]
    pub fn new() -> (Self, PyShdlcCommand) {
        (
            PyShdlcCmdFactoryReset,
            PyShdlcCommand::new(0x92, vec![], 2.0, 0, 0, 2.0),
        )
    }
}

#[pyclass(name = "ShdlcCmdEnterBootloader", extends = PyShdlcCommand)]
pub struct PyShdlcCmdEnterBootloader;

#[pymethods]
impl PyShdlcCmdEnterBootloader {
    #[new]
    pub fn new() -> (Self, PyShdlcCommand) {
        (
            PyShdlcCmdEnterBootloader,
            PyShdlcCommand::new(0xF3, vec![], 0.1, 0, 0, 2.0),
        )
    }
}

#[pyclass(name = "ShdlcCmdFirmwareUpdateStart", extends = PyShdlcCommand)]
pub struct PyShdlcCmdFirmwareUpdateStart;

#[pymethods]
impl PyShdlcCmdFirmwareUpdateStart {
    #[new]
    pub fn new() -> (Self, PyShdlcCommand) {
        (
            PyShdlcCmdFirmwareUpdateStart,
            PyShdlcCommand::new(0xF3, vec![0x01], 20.0, 0, 0, 0.0),
        )
    }
}

#[pyclass(name = "ShdlcCmdFirmwareUpdateData", extends = PyShdlcCommand)]
pub struct PyShdlcCmdFirmwareUpdateData;

#[pymethods]
impl PyShdlcCmdFirmwareUpdateData {
    #[new]
    pub fn new(data: Vec<u8>) -> (Self, PyShdlcCommand) {
        let mut d = vec![0x02];
        d.extend_from_slice(&data);
        (
            PyShdlcCmdFirmwareUpdateData,
            PyShdlcCommand::new(0xF3, d, 1.0, 0, 0, 0.0),
        )
    }
}

#[pyclass(name = "ShdlcCmdFirmwareUpdateStop", extends = PyShdlcCommand)]
pub struct PyShdlcCmdFirmwareUpdateStop;

#[pymethods]
impl PyShdlcCmdFirmwareUpdateStop {
    #[new]
    pub fn new(checksum: u8) -> (Self, PyShdlcCommand) {
        (
            PyShdlcCmdFirmwareUpdateStop,
            PyShdlcCommand::new(0xF3, vec![0x03, checksum], 1.0, 0, 0, 2.0),
        )
    }
}

use byteorder::ByteOrder;
