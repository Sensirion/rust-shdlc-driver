use crate::protocol::errors::ShdlcError;
use crate::python::errors::to_py_err;
use crate::transport::mock::MockTransport;
use crate::transport::runtime::get_runtime;
#[cfg(feature = "serial")]
use crate::transport::serial::AsyncSerialPort;
use crate::transport::tcp::AsyncTcpPort;
use crate::transport::traits::ShdlcTransport;
use crate::transport::transceiver::ShdlcTransceiver;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::time::Duration;

// ---------------------------------------------------------------------------
// Sync ShdlcSerialPort
// ---------------------------------------------------------------------------
#[cfg(feature = "serial")]
#[pyclass(name = "ShdlcSerialPort")]
pub struct PyShdlcSerialPort {
    port_name: String,
    baudrate: u32,
    additional_response_time: f64,
    serial_port: Option<AsyncSerialPort>,
}

#[cfg(feature = "serial")]
impl PyShdlcSerialPort {
    pub fn to_transport(&mut self) -> Result<Box<dyn ShdlcTransport>, ShdlcError> {
        let port = self
            .serial_port
            .take()
            .unwrap_or_else(|| AsyncSerialPort::new(&self.port_name, self.baudrate));
        Ok(Box::new(port))
    }
}

#[cfg(feature = "serial")]
#[pymethods]
impl PyShdlcSerialPort {
    #[new]
    #[pyo3(signature = (port, baudrate, additional_response_time=0.1, do_open=true))]
    pub fn new(
        py: Python<'_>,
        port: String,
        baudrate: u32,
        additional_response_time: f64,
        do_open: bool,
    ) -> PyResult<Self> {
        let mut port_obj = AsyncSerialPort::new(&port, baudrate);
        if do_open {
            py.allow_threads(|| port_obj.open())
                .map_err(|e| to_py_err(py, e))?;
        }

        Ok(Self {
            port_name: port,
            baudrate,
            additional_response_time,
            serial_port: Some(port_obj),
        })
    }

    pub fn __enter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    #[pyo3(signature = (_exc_type=None, _exc_val=None, _exc_tb=None))]
    pub fn __exit__(
        &mut self,
        py: Python<'_>,
        _exc_type: Option<&Bound<'_, PyAny>>,
        _exc_val: Option<&Bound<'_, PyAny>>,
        _exc_tb: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<()> {
        self.close(py)
    }

    #[getter]
    pub fn description(&self) -> String {
        format!("{}@{}", self.port_name, self.baudrate)
    }

    #[getter]
    pub fn bitrate(&self) -> u32 {
        self.baudrate
    }

    #[setter]
    pub fn set_bitrate(&mut self, py: Python<'_>, bitrate: u32) -> PyResult<()> {
        if let Some(ref mut port) = self.serial_port {
            py.allow_threads(|| get_runtime().block_on(port.set_bitrate(bitrate)))
                .map_err(|e| to_py_err(py, e))?;
        }
        self.baudrate = bitrate;
        Ok(())
    }

    #[getter]
    pub fn additional_response_time(&self) -> f64 {
        self.additional_response_time
    }

    #[setter]
    pub fn set_additional_response_time(&mut self, val: f64) {
        self.additional_response_time = val;
    }

    #[getter]
    pub fn is_open(&self) -> bool {
        self.serial_port
            .as_ref()
            .map(|p| p.is_open())
            .unwrap_or(false)
    }

    pub fn open(&mut self, py: Python<'_>) -> PyResult<()> {
        if let Some(ref mut port) = self.serial_port {
            py.allow_threads(|| port.open())
                .map_err(|e| to_py_err(py, e))?;
        } else {
            let mut port = AsyncSerialPort::new(&self.port_name, self.baudrate);
            py.allow_threads(|| port.open())
                .map_err(|e| to_py_err(py, e))?;
            self.serial_port = Some(port);
        }
        Ok(())
    }

    pub fn close(&mut self, py: Python<'_>) -> PyResult<()> {
        if let Some(ref mut port) = self.serial_port {
            py.allow_threads(|| {
                let _ = get_runtime().block_on(port.close());
            });
        }
        Ok(())
    }

    pub fn transceive<'py>(
        &mut self,
        py: Python<'py>,
        slave_address: u8,
        command_id: u8,
        data: Vec<u8>,
        response_timeout: f64,
    ) -> PyResult<(u8, u8, u8, Bound<'py, PyBytes>)> {
        let port = self
            .serial_port
            .as_mut()
            .ok_or_else(|| to_py_err(py, ShdlcError::PortError("Port is not open".to_string())))?;

        let timeout_dur = Duration::from_secs_f64(response_timeout);
        let extra_dur = Duration::from_secs_f64(self.additional_response_time);

        let miso = py
            .allow_threads(|| {
                get_runtime().block_on(ShdlcTransceiver::transceive(
                    port,
                    slave_address,
                    command_id,
                    &data,
                    timeout_dur,
                    extra_dur,
                ))
            })
            .map_err(|e| to_py_err(py, e))?;

        let bytes = PyBytes::new(py, &miso.data);
        Ok((miso.slave_address, miso.command_id, miso.state, bytes))
    }
}

// ---------------------------------------------------------------------------
// Sync ShdlcTcpPort
// ---------------------------------------------------------------------------
#[pyclass(name = "ShdlcTcpPort")]
pub struct PyShdlcTcpPort {
    ip: String,
    port: u16,
    socket_timeout: f64,
    tcp_port: Option<AsyncTcpPort>,
}

impl PyShdlcTcpPort {
    pub fn to_transport(&mut self) -> Result<Box<dyn ShdlcTransport>, ShdlcError> {
        let tcp = self
            .tcp_port
            .take()
            .unwrap_or_else(|| AsyncTcpPort::new(&self.ip, self.port));
        Ok(Box::new(tcp))
    }
}

#[pymethods]
impl PyShdlcTcpPort {
    #[new]
    #[pyo3(signature = (ip, port, socket_timeout=5.0, do_open=true))]
    pub fn new(
        py: Python<'_>,
        ip: String,
        port: u16,
        socket_timeout: f64,
        do_open: bool,
    ) -> PyResult<Self> {
        let mut tcp = AsyncTcpPort::new(&ip, port);
        if do_open {
            py.allow_threads(|| get_runtime().block_on(tcp.open()))
                .map_err(|e| to_py_err(py, e))?;
        }

        Ok(Self {
            ip,
            port,
            socket_timeout,
            tcp_port: Some(tcp),
        })
    }

    pub fn __enter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    #[pyo3(signature = (_exc_type=None, _exc_val=None, _exc_tb=None))]
    pub fn __exit__(
        &mut self,
        py: Python<'_>,
        _exc_type: Option<&Bound<'_, PyAny>>,
        _exc_val: Option<&Bound<'_, PyAny>>,
        _exc_tb: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<()> {
        self.close(py)
    }

    #[getter]
    pub fn description(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    #[getter]
    pub fn bitrate(&self) -> u32 {
        115200
    }

    #[setter]
    pub fn set_bitrate(&mut self, py: Python<'_>, _bitrate: u32) -> PyResult<()> {
        Err(to_py_err(
            py,
            ShdlcError::PortError("Changing bitrate is not supported on TCP port".to_string()),
        ))
    }

    #[getter]
    pub fn socket_timeout(&self) -> f64 {
        self.socket_timeout
    }

    #[setter]
    pub fn set_socket_timeout(&mut self, timeout: f64) {
        self.socket_timeout = timeout;
    }

    #[getter]
    pub fn is_open(&self) -> bool {
        self.tcp_port.as_ref().map(|p| p.is_open()).unwrap_or(false)
    }

    pub fn open(&mut self, py: Python<'_>) -> PyResult<()> {
        if let Some(ref mut tcp) = self.tcp_port {
            py.allow_threads(|| get_runtime().block_on(tcp.open()))
                .map_err(|e| to_py_err(py, e))?;
        } else {
            let mut tcp = AsyncTcpPort::new(&self.ip, self.port);
            py.allow_threads(|| get_runtime().block_on(tcp.open()))
                .map_err(|e| to_py_err(py, e))?;
            self.tcp_port = Some(tcp);
        }
        Ok(())
    }

    pub fn close(&mut self, py: Python<'_>) -> PyResult<()> {
        if let Some(ref mut tcp) = self.tcp_port {
            py.allow_threads(|| {
                let _ = get_runtime().block_on(tcp.close());
            });
        }
        Ok(())
    }

    pub fn transceive<'py>(
        &mut self,
        py: Python<'py>,
        slave_address: u8,
        command_id: u8,
        data: Vec<u8>,
        response_timeout: f64,
    ) -> PyResult<(u8, u8, u8, Bound<'py, PyBytes>)> {
        let tcp = self
            .tcp_port
            .as_mut()
            .ok_or_else(|| to_py_err(py, ShdlcError::PortError("Port is not open".to_string())))?;

        let timeout_dur = Duration::from_secs_f64(self.socket_timeout + response_timeout);

        let miso = py
            .allow_threads(|| {
                get_runtime().block_on(ShdlcTransceiver::transceive(
                    tcp,
                    slave_address,
                    command_id,
                    &data,
                    timeout_dur,
                    Duration::from_millis(0),
                ))
            })
            .map_err(|e| to_py_err(py, e))?;

        let bytes = PyBytes::new(py, &miso.data);
        Ok((miso.slave_address, miso.command_id, miso.state, bytes))
    }
}

// ---------------------------------------------------------------------------
// Sync/Async Mock Port for Python tests
// ---------------------------------------------------------------------------
#[pyclass(name = "ShdlcMockPort")]
#[derive(Clone)]
pub struct PyShdlcMockPort {
    pub inner: MockTransport,
}

impl PyShdlcMockPort {
    pub fn to_transport(&self) -> Box<dyn ShdlcTransport> {
        Box::new(self.inner.clone())
    }
}

#[pymethods]
impl PyShdlcMockPort {
    #[new]
    #[pyo3(signature = (bitrate=115200))]
    pub fn new(_py: Python<'_>, bitrate: u32) -> Self {
        Self {
            inner: MockTransport::with_bitrate(bitrate),
        }
    }

    pub fn push_rx_data(&self, data: &[u8]) {
        self.inner.push_rx_data(data);
    }

    pub fn get_written_data<'py>(&self, py: Python<'py>) -> Vec<Bound<'py, PyBytes>> {
        self.inner
            .get_written_data()
            .into_iter()
            .map(|chunk| PyBytes::new(py, &chunk))
            .collect()
    }

    pub fn clear_written_data(&self) {
        self.inner.clear_written_data();
    }

    pub fn set_timeout_on_read(&self, timeout: bool) {
        self.inner.set_timeout_on_read(timeout);
    }

    #[getter]
    pub fn bitrate(&self, py: Python<'_>) -> PyResult<u32> {
        self.inner.bitrate().map_err(|e| to_py_err(py, e))
    }

    #[setter]
    pub fn set_bitrate(&mut self, py: Python<'_>, bitrate: u32) -> PyResult<()> {
        let mut inner = self.inner.clone();
        py.allow_threads(|| get_runtime().block_on(inner.set_bitrate(bitrate)))
            .map_err(|e| to_py_err(py, e))
    }

    #[getter]
    pub fn description(&self) -> String {
        self.inner.description()
    }

    #[getter]
    pub fn is_open(&self) -> bool {
        self.inner.is_open()
    }

    pub fn close(&mut self, py: Python<'_>) -> PyResult<()> {
        let mut inner = self.inner.clone();
        py.allow_threads(|| get_runtime().block_on(inner.close()))
            .map_err(|e| to_py_err(py, e))
    }

    pub fn transceive<'py>(
        &mut self,
        py: Python<'py>,
        slave_address: u8,
        command_id: u8,
        data: Vec<u8>,
        response_timeout: f64,
    ) -> PyResult<(u8, u8, u8, Bound<'py, PyBytes>)> {
        let timeout_dur = Duration::from_secs_f64(response_timeout);
        let mut inner = self.inner.clone();
        let miso = py
            .allow_threads(|| {
                get_runtime().block_on(ShdlcTransceiver::transceive(
                    &mut inner,
                    slave_address,
                    command_id,
                    &data,
                    timeout_dur,
                    Duration::from_millis(10),
                ))
            })
            .map_err(|e| to_py_err(py, e))?;

        let bytes = PyBytes::new(py, &miso.data);
        Ok((miso.slave_address, miso.command_id, miso.state, bytes))
    }
}
