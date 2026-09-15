use crate::connection::AsyncShdlcConnection;
use crate::protocol::commands::RawShdlcCommand;
use crate::python::commands::PyShdlcCommand;
use crate::python::errors::to_py_err;
#[cfg(feature = "serial")]
use crate::python::ports::PyShdlcSerialPort;
use crate::python::ports::{PyShdlcMockPort, PyShdlcTcpPort};
use crate::transport::runtime::get_runtime;
use crate::transport::traits::ShdlcTransport;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::sync::Arc;
use std::time::Duration;

#[pyclass(name = "ShdlcConnection")]
#[derive(Clone)]
pub struct PyShdlcConnection {
    pub async_conn: AsyncShdlcConnection,
    pub port_obj: Arc<PyObject>,
}

#[pymethods]
impl PyShdlcConnection {
    #[new]
    pub fn new(py: Python<'_>, port: Bound<'_, PyAny>) -> PyResult<Self> {
        let transport: Box<dyn ShdlcTransport> = {
            #[cfg(feature = "serial")]
            if let Ok(mut ser) = port.extract::<PyRefMut<'_, PyShdlcSerialPort>>() {
                ser.to_transport().map_err(|e| to_py_err(py, e))?
            } else if let Ok(mut tcp) = port.extract::<PyRefMut<'_, PyShdlcTcpPort>>() {
                tcp.to_transport().map_err(|e| to_py_err(py, e))?
            } else if let Ok(mock) = port.extract::<PyRef<'_, PyShdlcMockPort>>() {
                mock.to_transport()
            } else {
                return Err(to_py_err(
                    py,
                    crate::protocol::errors::ShdlcError::PortError(
                        "Unsupported port object passed to ShdlcConnection".to_string(),
                    ),
                ));
            }

            #[cfg(not(feature = "serial"))]
            if let Ok(mut tcp) = port.extract::<PyRefMut<'_, PyShdlcTcpPort>>() {
                tcp.to_transport().map_err(|e| to_py_err(py, e))?
            } else if let Ok(mock) = port.extract::<PyRef<'_, PyShdlcMockPort>>() {
                mock.to_transport()
            } else {
                return Err(to_py_err(
                    py,
                    crate::protocol::errors::ShdlcError::PortError(
                        "Unsupported port object passed to ShdlcConnection".to_string(),
                    ),
                ));
            }
        };

        let async_conn = AsyncShdlcConnection::new(transport);

        Ok(Self {
            async_conn,
            port_obj: Arc::new(port.unbind()),
        })
    }

    #[getter]
    pub fn port(&self, py: Python<'_>) -> PyObject {
        self.port_obj.as_ref().clone_ref(py)
    }

    pub fn transceive<'py>(
        &self,
        py: Python<'py>,
        slave_address: u8,
        command_id: u8,
        data: Vec<u8>,
        response_timeout: f64,
    ) -> PyResult<(Bound<'py, PyBytes>, bool)> {
        let timeout_dur = Duration::from_secs_f64(response_timeout);
        let (rx_data, error_state) = get_runtime()
            .block_on(
                self.async_conn
                    .transceive(slave_address, command_id, &data, timeout_dur),
            )
            .map_err(|e| to_py_err(py, e))?;
        let bytes = PyBytes::new(py, &rx_data);
        Ok((bytes, error_state))
    }

    #[pyo3(signature = (slave_address, command, wait_post_process=true))]
    pub fn execute(
        &self,
        py: Python<'_>,
        slave_address: u8,
        command: Bound<'_, PyAny>,
        wait_post_process: bool,
    ) -> PyResult<(PyObject, bool)> {
        let (id, data, max_response_time, min_len, max_len, post_proc) =
            if let Ok(cmd) = command.extract::<PyRef<'_, PyShdlcCommand>>() {
                (
                    cmd.id,
                    cmd.data.clone(),
                    cmd.max_response_time,
                    cmd.min_response_length,
                    cmd.max_response_length,
                    cmd.post_processing_time,
                )
            } else {
                let id: u8 = command.getattr("id")?.extract()?;
                let data: Vec<u8> = command.getattr("data")?.extract()?;
                let max_resp: f64 = command.getattr("max_response_time")?.extract()?;
                let min_l: usize = command
                    .getattr("min_response_length")
                    .map(|v| v.extract().unwrap_or(0))
                    .unwrap_or(0);
                let max_l: usize = command
                    .getattr("max_response_length")
                    .map(|v| v.extract().unwrap_or(255))
                    .unwrap_or(255);
                let post_p: f64 = command
                    .getattr("post_processing_time")
                    .map(|v| v.extract().unwrap_or(0.0))
                    .unwrap_or(0.0);
                (id, data, max_resp, min_l, max_l, post_p)
            };

        let raw_cmd = RawShdlcCommand::new(id, &data, Duration::from_secs_f64(max_response_time))
            .with_response_lengths(min_len, max_len)
            .with_post_processing_time(Duration::from_secs_f64(post_proc));

        let (rx_bytes, error_state) = get_runtime()
            .block_on(
                self.async_conn
                    .execute(slave_address, &raw_cmd, wait_post_process),
            )
            .map_err(|e| to_py_err(py, e))?;

        let py_rx_bytes = PyBytes::new(py, &rx_bytes);
        let interpreted = if command.hasattr("interpret_response")? {
            command
                .call_method1("interpret_response", (&py_rx_bytes,))?
                .unbind()
        } else {
            py_rx_bytes.unbind().into()
        };

        Ok((interpreted, error_state))
    }
}

// ---------------------------------------------------------------------------
// Async ShdlcConnection for Python asyncio
// ---------------------------------------------------------------------------
#[pyclass(name = "AsyncShdlcConnection")]
#[derive(Clone)]
pub struct PyAsyncShdlcConnection {
    pub async_conn: AsyncShdlcConnection,
    pub port_obj: Arc<PyObject>,
}

#[pymethods]
impl PyAsyncShdlcConnection {
    #[new]
    pub fn new(py: Python<'_>, port: Bound<'_, PyAny>) -> PyResult<Self> {
        let transport: Box<dyn ShdlcTransport> = {
            #[cfg(feature = "serial")]
            if let Ok(mut ser) = port.extract::<PyRefMut<'_, PyShdlcSerialPort>>() {
                ser.to_transport().map_err(|e| to_py_err(py, e))?
            } else if let Ok(mut tcp) = port.extract::<PyRefMut<'_, PyShdlcTcpPort>>() {
                tcp.to_transport().map_err(|e| to_py_err(py, e))?
            } else if let Ok(mock) = port.extract::<PyRef<'_, PyShdlcMockPort>>() {
                mock.to_transport()
            } else {
                return Err(to_py_err(
                    py,
                    crate::protocol::errors::ShdlcError::PortError(
                        "Unsupported port object".to_string(),
                    ),
                ));
            }

            #[cfg(not(feature = "serial"))]
            if let Ok(mut tcp) = port.extract::<PyRefMut<'_, PyShdlcTcpPort>>() {
                tcp.to_transport().map_err(|e| to_py_err(py, e))?
            } else if let Ok(mock) = port.extract::<PyRef<'_, PyShdlcMockPort>>() {
                mock.to_transport()
            } else {
                return Err(to_py_err(
                    py,
                    crate::protocol::errors::ShdlcError::PortError(
                        "Unsupported port object".to_string(),
                    ),
                ));
            }
        };

        let async_conn = AsyncShdlcConnection::new(transport);
        Ok(Self {
            async_conn,
            port_obj: Arc::new(port.unbind()),
        })
    }

    #[getter]
    pub fn port(&self, py: Python<'_>) -> PyObject {
        self.port_obj.as_ref().clone_ref(py)
    }

    pub fn transceive<'py>(
        &self,
        py: Python<'py>,
        slave_address: u8,
        command_id: u8,
        data: Vec<u8>,
        response_timeout: f64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let conn = self.async_conn.clone();
        let timeout_dur = Duration::from_secs_f64(response_timeout);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let (rx_data, error_state) = conn
                .transceive(slave_address, command_id, &data, timeout_dur)
                .await
                .map_err(|e| Python::with_gil(|py| to_py_err(py, e)))?;

            Python::with_gil(|py| {
                let bytes = PyBytes::new(py, &rx_data).unbind();
                Ok((bytes, error_state))
            })
        })
    }

    #[pyo3(signature = (slave_address, command, wait_post_process=true))]
    pub fn execute<'py>(
        &self,
        py: Python<'py>,
        slave_address: u8,
        command: Bound<'py, PyAny>,
        wait_post_process: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let (id, data, max_response_time, min_len, max_len, post_proc) =
            if let Ok(cmd) = command.extract::<PyRef<'_, PyShdlcCommand>>() {
                (
                    cmd.id,
                    cmd.data.clone(),
                    cmd.max_response_time,
                    cmd.min_response_length,
                    cmd.max_response_length,
                    cmd.post_processing_time,
                )
            } else {
                let id: u8 = command.getattr("id")?.extract()?;
                let data: Vec<u8> = command.getattr("data")?.extract()?;
                let max_resp: f64 = command.getattr("max_response_time")?.extract()?;
                let min_l: usize = command
                    .getattr("min_response_length")
                    .map(|v| v.extract().unwrap_or(0))
                    .unwrap_or(0);
                let max_l: usize = command
                    .getattr("max_response_length")
                    .map(|v| v.extract().unwrap_or(255))
                    .unwrap_or(255);
                let post_p: f64 = command
                    .getattr("post_processing_time")
                    .map(|v| v.extract().unwrap_or(0.0))
                    .unwrap_or(0.0);
                (id, data, max_resp, min_l, max_l, post_p)
            };

        let cmd_py: PyObject = command.unbind();
        let conn = self.async_conn.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let raw_cmd =
                RawShdlcCommand::new(id, &data, Duration::from_secs_f64(max_response_time))
                    .with_response_lengths(min_len, max_len)
                    .with_post_processing_time(Duration::from_secs_f64(post_proc));

            let (rx_bytes, error_state) = conn
                .execute(slave_address, &raw_cmd, wait_post_process)
                .await
                .map_err(|e| Python::with_gil(|py| to_py_err(py, e)))?;

            Python::with_gil(|py| -> PyResult<(PyObject, bool)> {
                let py_rx_bytes = PyBytes::new(py, &rx_bytes);
                let bound_cmd = cmd_py.bind(py);
                let interpreted: PyObject = if bound_cmd.hasattr("interpret_response")? {
                    bound_cmd
                        .call_method1("interpret_response", (py_rx_bytes,))?
                        .unbind()
                } else {
                    py_rx_bytes.unbind().into()
                };
                Ok((interpreted, error_state))
            })
        })
    }
}
