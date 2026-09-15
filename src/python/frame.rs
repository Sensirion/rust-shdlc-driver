use crate::protocol::frame::{ShdlcMisoFrameBuilder, ShdlcMosiFrame};
use crate::python::errors::to_py_err;
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyBytes};

#[pyclass(name = "ShdlcSerialMosiFrameBuilder")]
pub struct PyShdlcSerialMosiFrameBuilder {
    slave_address: u8,
    command_id: u8,
    data: Vec<u8>,
}

#[pymethods]
impl PyShdlcSerialMosiFrameBuilder {
    #[new]
    pub fn new(slave_address: u8, command_id: u8, data: Vec<u8>) -> Self {
        Self {
            slave_address,
            command_id,
            data,
        }
    }

    pub fn to_bytes<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let frame = ShdlcMosiFrame::new(self.slave_address, self.command_id, &self.data);
        PyBytes::new(py, &frame.to_bytes())
    }
}

#[pyclass(name = "ShdlcSerialMisoFrameBuilder")]
pub struct PyShdlcSerialMisoFrameBuilder {
    builder: ShdlcMisoFrameBuilder,
}

impl Default for PyShdlcSerialMisoFrameBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[pymethods]
impl PyShdlcSerialMisoFrameBuilder {
    #[new]
    pub fn new() -> Self {
        Self {
            builder: ShdlcMisoFrameBuilder::new(),
        }
    }

    #[getter]
    pub fn data<'py>(&self, py: Python<'py>) -> Bound<'py, PyByteArray> {
        PyByteArray::new(py, self.builder.data())
    }

    #[getter]
    pub fn start_received(&self) -> bool {
        self.builder.start_received()
    }

    pub fn add_data(&mut self, py: Python<'_>, data: &[u8]) -> PyResult<bool> {
        self.builder.add_data(data).map_err(|e| to_py_err(py, e))
    }

    pub fn interpret_data<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(u8, u8, u8, Bound<'py, PyBytes>)> {
        let frame = self
            .builder
            .interpret_data()
            .map_err(|e| to_py_err(py, e))?;
        let bytes = PyBytes::new(py, &frame.data);
        Ok((frame.slave_address, frame.command_id, frame.state, bytes))
    }
}
