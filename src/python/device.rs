use crate::device::AsyncShdlcDevice;
use crate::protocol::commands::*;
use crate::python::connection::{PyAsyncShdlcConnection, PyShdlcConnection};
use crate::python::errors::to_py_err;
use crate::python::types::PyVersion;
use crate::transport::runtime::get_runtime;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[pyclass(name = "ShdlcDeviceBase", subclass)]
#[derive(Clone)]
pub struct PyShdlcDeviceBase {
    pub conn_obj: PyShdlcConnection,
    pub slave_address: u8,
    pub last_error_flag: bool,
    pub device_errors: HashMap<u8, String>,
}

#[pymethods]
impl PyShdlcDeviceBase {
    #[new]
    pub fn new(connection: PyShdlcConnection, slave_address: u8) -> Self {
        Self {
            conn_obj: connection,
            slave_address,
            last_error_flag: false,
            device_errors: HashMap::new(),
        }
    }

    #[getter]
    pub fn connection(&self) -> PyShdlcConnection {
        self.conn_obj.clone()
    }

    #[getter]
    pub fn slave_address(&self) -> u8 {
        self.slave_address
    }

    #[getter]
    pub fn last_error_flag(&self) -> bool {
        self.last_error_flag
    }

    pub fn execute(&mut self, py: Python<'_>, command: Bound<'_, PyAny>) -> PyResult<PyObject> {
        let (data, err_flag) = self
            .conn_obj
            .execute(py, self.slave_address, command, true)?;
        self.last_error_flag = err_flag;
        Ok(data)
    }
}

// ---------------------------------------------------------------------------
// Sync ShdlcDevice
// ---------------------------------------------------------------------------
#[pyclass(name = "ShdlcDevice", extends = PyShdlcDeviceBase)]
pub struct PyShdlcDevice;

#[pymethods]
impl PyShdlcDevice {
    #[new]
    pub fn new(connection: PyShdlcConnection, slave_address: u8) -> (Self, PyShdlcDeviceBase) {
        (
            PyShdlcDevice,
            PyShdlcDeviceBase::new(connection, slave_address),
        )
    }

    #[pyo3(signature = (as_int=false))]
    pub fn get_product_type(
        mut slf: PyRefMut<'_, Self>,
        py: Python<'_>,
        as_int: bool,
    ) -> PyResult<PyObject> {
        let base = slf.as_mut();
        let (s, err_flag) = get_runtime()
            .block_on(
                base.conn_obj
                    .async_conn
                    .execute(base.slave_address, &GetProductType, true),
            )
            .map_err(|e| to_py_err(py, e))?;
        base.last_error_flag = err_flag;

        if as_int {
            let val = u32::from_str_radix(&s, 16).unwrap_or(0);
            Ok(val.into_pyobject(py)?.unbind().into())
        } else {
            Ok(s.into_pyobject(py)?.unbind().into())
        }
    }

    pub fn get_product_subtype(mut slf: PyRefMut<'_, Self>, py: Python<'_>) -> PyResult<u8> {
        let base = slf.as_mut();
        let (sub, err_flag) = get_runtime()
            .block_on(base.conn_obj.async_conn.execute(
                base.slave_address,
                &GetProductSubType,
                true,
            ))
            .map_err(|e| to_py_err(py, e))?;
        base.last_error_flag = err_flag;
        Ok(sub)
    }

    pub fn get_product_name(mut slf: PyRefMut<'_, Self>, py: Python<'_>) -> PyResult<String> {
        let base = slf.as_mut();
        let (name, err_flag) = get_runtime()
            .block_on(
                base.conn_obj
                    .async_conn
                    .execute(base.slave_address, &GetProductName, true),
            )
            .map_err(|e| to_py_err(py, e))?;
        base.last_error_flag = err_flag;
        Ok(name)
    }

    pub fn get_article_code(mut slf: PyRefMut<'_, Self>, py: Python<'_>) -> PyResult<String> {
        let base = slf.as_mut();
        let (code, err_flag) = get_runtime()
            .block_on(
                base.conn_obj
                    .async_conn
                    .execute(base.slave_address, &GetArticleCode, true),
            )
            .map_err(|e| to_py_err(py, e))?;
        base.last_error_flag = err_flag;
        Ok(code)
    }

    pub fn get_serial_number(mut slf: PyRefMut<'_, Self>, py: Python<'_>) -> PyResult<String> {
        let base = slf.as_mut();
        let (sn, err_flag) = get_runtime()
            .block_on(
                base.conn_obj
                    .async_conn
                    .execute(base.slave_address, &GetSerialNumber, true),
            )
            .map_err(|e| to_py_err(py, e))?;
        base.last_error_flag = err_flag;
        Ok(sn)
    }

    pub fn get_version(mut slf: PyRefMut<'_, Self>, py: Python<'_>) -> PyResult<PyVersion> {
        let base = slf.as_mut();
        let (v, err_flag) = get_runtime()
            .block_on(
                base.conn_obj
                    .async_conn
                    .execute(base.slave_address, &GetVersion, true),
            )
            .map_err(|e| to_py_err(py, e))?;
        base.last_error_flag = err_flag;
        Ok(v.into())
    }

    #[pyo3(signature = (clear=true, as_exception=false))]
    pub fn get_error_state(
        mut slf: PyRefMut<'_, Self>,
        py: Python<'_>,
        clear: bool,
        as_exception: bool,
    ) -> PyResult<(u32, PyObject)> {
        let base = slf.as_mut();
        let ((state, last_err), err_flag) = get_runtime()
            .block_on(base.conn_obj.async_conn.execute(
                base.slave_address,
                &GetErrorState::new(clear),
                true,
            ))
            .map_err(|e| to_py_err(py, e))?;
        base.last_error_flag = err_flag;

        if as_exception {
            if last_err != 0 {
                let err = crate::protocol::errors::ShdlcError::device_error(last_err);
                let py_err = to_py_err(py, err);
                Ok((state, py_err.value(py).clone().unbind().into()))
            } else {
                Ok((state, py.None()))
            }
        } else {
            Ok((state, last_err.into_pyobject(py)?.unbind().into()))
        }
    }

    pub fn get_slave_address(mut slf: PyRefMut<'_, Self>, py: Python<'_>) -> PyResult<u8> {
        let base = slf.as_mut();
        let (addr, err_flag) = get_runtime()
            .block_on(
                base.conn_obj
                    .async_conn
                    .execute(base.slave_address, &GetSlaveAddress, true),
            )
            .map_err(|e| to_py_err(py, e))?;
        base.last_error_flag = err_flag;
        Ok(addr)
    }

    #[pyo3(signature = (slave_address, update_driver=true))]
    pub fn set_slave_address(
        mut slf: PyRefMut<'_, Self>,
        py: Python<'_>,
        slave_address: u8,
        update_driver: bool,
    ) -> PyResult<()> {
        let base = slf.as_mut();
        let (_, err_flag) = get_runtime()
            .block_on(base.conn_obj.async_conn.execute(
                base.slave_address,
                &SetSlaveAddress::new(slave_address),
                true,
            ))
            .map_err(|e| to_py_err(py, e))?;
        base.last_error_flag = err_flag;
        if update_driver {
            base.slave_address = slave_address;
        }
        Ok(())
    }

    pub fn get_baudrate(mut slf: PyRefMut<'_, Self>, py: Python<'_>) -> PyResult<u32> {
        let base = slf.as_mut();
        let (baud, err_flag) = get_runtime()
            .block_on(
                base.conn_obj
                    .async_conn
                    .execute(base.slave_address, &GetBaudrate, true),
            )
            .map_err(|e| to_py_err(py, e))?;
        base.last_error_flag = err_flag;
        Ok(baud)
    }

    #[pyo3(signature = (baudrate, update_driver=true))]
    pub fn set_baudrate(
        mut slf: PyRefMut<'_, Self>,
        py: Python<'_>,
        baudrate: u32,
        update_driver: bool,
    ) -> PyResult<()> {
        let base = slf.as_mut();
        let (_, err_flag) = get_runtime()
            .block_on(base.conn_obj.async_conn.execute(
                base.slave_address,
                &SetBaudrate::new(baudrate),
                true,
            ))
            .map_err(|e| to_py_err(py, e))?;
        base.last_error_flag = err_flag;
        if update_driver {
            let transport_arc = base.conn_obj.async_conn.transport();
            let _ = get_runtime().block_on(async move {
                let mut transport = transport_arc.lock().await;
                transport.set_bitrate(baudrate).await
            });
        }
        Ok(())
    }

    pub fn get_reply_delay(mut slf: PyRefMut<'_, Self>, py: Python<'_>) -> PyResult<u16> {
        let base = slf.as_mut();
        let (delay, err_flag) = get_runtime()
            .block_on(
                base.conn_obj
                    .async_conn
                    .execute(base.slave_address, &GetReplyDelay, true),
            )
            .map_err(|e| to_py_err(py, e))?;
        base.last_error_flag = err_flag;
        Ok(delay)
    }

    pub fn set_reply_delay(
        mut slf: PyRefMut<'_, Self>,
        py: Python<'_>,
        reply_delay: u16,
    ) -> PyResult<()> {
        let base = slf.as_mut();
        let (_, err_flag) = get_runtime()
            .block_on(base.conn_obj.async_conn.execute(
                base.slave_address,
                &SetReplyDelay::new(reply_delay),
                true,
            ))
            .map_err(|e| to_py_err(py, e))?;
        base.last_error_flag = err_flag;
        Ok(())
    }

    pub fn get_system_up_time(mut slf: PyRefMut<'_, Self>, py: Python<'_>) -> PyResult<u32> {
        let base = slf.as_mut();
        let (uptime, err_flag) = get_runtime()
            .block_on(
                base.conn_obj
                    .async_conn
                    .execute(base.slave_address, &GetSystemUpTime, true),
            )
            .map_err(|e| to_py_err(py, e))?;
        base.last_error_flag = err_flag;
        Ok(uptime)
    }

    pub fn device_reset(mut slf: PyRefMut<'_, Self>, py: Python<'_>) -> PyResult<()> {
        let base = slf.as_mut();
        let (_, err_flag) = get_runtime()
            .block_on(
                base.conn_obj
                    .async_conn
                    .execute(base.slave_address, &DeviceReset, true),
            )
            .map_err(|e| to_py_err(py, e))?;
        base.last_error_flag = err_flag;
        Ok(())
    }

    pub fn factory_reset(mut slf: PyRefMut<'_, Self>, py: Python<'_>) -> PyResult<()> {
        let base = slf.as_mut();
        let (_, err_flag) = get_runtime()
            .block_on(
                base.conn_obj
                    .async_conn
                    .execute(base.slave_address, &FactoryReset, true),
            )
            .map_err(|e| to_py_err(py, e))?;
        base.last_error_flag = err_flag;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Async ShdlcDevice
// ---------------------------------------------------------------------------
#[pyclass(name = "AsyncShdlcDevice")]
#[derive(Clone)]
pub struct PyAsyncShdlcDevice {
    pub inner: Arc<tokio::sync::Mutex<AsyncShdlcDevice>>,
    pub conn_obj: PyAsyncShdlcConnection,
}

#[pymethods]
impl PyAsyncShdlcDevice {
    #[new]
    pub fn new(connection: PyAsyncShdlcConnection, slave_address: u8) -> Self {
        let dev = AsyncShdlcDevice::new(connection.async_conn.clone(), slave_address);
        Self {
            inner: Arc::new(tokio::sync::Mutex::new(dev)),
            conn_obj: connection,
        }
    }

    #[getter]
    pub fn connection(&self) -> PyAsyncShdlcConnection {
        self.conn_obj.clone()
    }

    #[pyo3(signature = (as_int=false))]
    pub fn get_product_type<'py>(
        &self,
        py: Python<'py>,
        as_int: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let dev_arc = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut dev = dev_arc.lock().await;
            let s = dev
                .get_product_type()
                .await
                .map_err(|e| Python::with_gil(|py| to_py_err(py, e)))?;
            Python::with_gil(|py| -> PyResult<PyObject> {
                if as_int {
                    let val = u32::from_str_radix(&s, 16).unwrap_or(0);
                    Ok(val.into_pyobject(py)?.unbind().into())
                } else {
                    Ok(s.into_pyobject(py)?.unbind().into())
                }
            })
        })
    }

    pub fn get_product_name<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let dev_arc = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut dev = dev_arc.lock().await;
            let s = dev
                .get_product_name()
                .await
                .map_err(|e| Python::with_gil(|py| to_py_err(py, e)))?;
            Ok(s)
        })
    }

    pub fn get_serial_number<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let dev_arc = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut dev = dev_arc.lock().await;
            let s = dev
                .get_serial_number()
                .await
                .map_err(|e| Python::with_gil(|py| to_py_err(py, e)))?;
            Ok(s)
        })
    }

    pub fn get_version<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let dev_arc = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut dev = dev_arc.lock().await;
            let v = dev
                .get_version()
                .await
                .map_err(|e| Python::with_gil(|py| to_py_err(py, e)))?;
            let py_v: PyVersion = v.into();
            Ok(py_v)
        })
    }

    #[getter(slave_address)]
    pub fn slave_address_prop(&self) -> u8 {
        self.inner
            .try_lock()
            .map(|d| d.slave_address())
            .unwrap_or(0)
    }

    #[getter(last_error_flag)]
    pub fn last_error_flag_prop(&self) -> bool {
        self.inner
            .try_lock()
            .map(|d| d.last_error_flag())
            .unwrap_or(false)
    }

    pub fn execute<'py>(
        &self,
        py: Python<'py>,
        command: Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let dev_arc = self.inner.clone();
        let cmd_py: PyObject = command.unbind();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let (id, data, max_resp, min_l, max_l, post_p) =
                Python::with_gil(|py| -> PyResult<_> {
                    let bound_cmd = cmd_py.bind(py);
                    if let Ok(cmd) =
                        bound_cmd.extract::<PyRef<'_, crate::python::commands::PyShdlcCommand>>()
                    {
                        Ok((
                            cmd.id,
                            cmd.data.clone(),
                            cmd.max_response_time,
                            cmd.min_response_length,
                            cmd.max_response_length,
                            cmd.post_processing_time,
                        ))
                    } else {
                        let id: u8 = bound_cmd.getattr("id")?.extract()?;
                        let data: Vec<u8> = bound_cmd.getattr("data")?.extract()?;
                        let max_resp: f64 = bound_cmd.getattr("max_response_time")?.extract()?;
                        let min_l: usize = bound_cmd
                            .getattr("min_response_length")
                            .map(|v| v.extract().unwrap_or(0))
                            .unwrap_or(0);
                        let max_l: usize = bound_cmd
                            .getattr("max_response_length")
                            .map(|v| v.extract().unwrap_or(255))
                            .unwrap_or(255);
                        let post_p: f64 = bound_cmd
                            .getattr("post_processing_time")
                            .map(|v| v.extract().unwrap_or(0.0))
                            .unwrap_or(0.0);
                        Ok((id, data, max_resp, min_l, max_l, post_p))
                    }
                })?;

            let raw_cmd =
                RawShdlcCommand::new(id, &data, std::time::Duration::from_secs_f64(max_resp))
                    .with_response_lengths(min_l, max_l)
                    .with_post_processing_time(std::time::Duration::from_secs_f64(post_p));

            let mut dev = dev_arc.lock().await;
            let rx_bytes = dev
                .execute_raw(&raw_cmd)
                .await
                .map_err(|e| Python::with_gil(|py| to_py_err(py, e)))?;

            Python::with_gil(|py| -> PyResult<PyObject> {
                let py_rx_bytes = pyo3::types::PyBytes::new(py, &rx_bytes);
                let bound_cmd = cmd_py.bind(py);
                if bound_cmd.hasattr("interpret_response")? {
                    Ok(bound_cmd
                        .call_method1("interpret_response", (py_rx_bytes,))?
                        .unbind())
                } else {
                    Ok(py_rx_bytes.unbind().into())
                }
            })
        })
    }

    pub fn get_product_subtype<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let dev_arc = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut dev = dev_arc.lock().await;
            let sub = dev
                .get_product_subtype()
                .await
                .map_err(|e| Python::with_gil(|py| to_py_err(py, e)))?;
            Ok(sub)
        })
    }

    pub fn get_article_code<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let dev_arc = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut dev = dev_arc.lock().await;
            let s = dev
                .get_article_code()
                .await
                .map_err(|e| Python::with_gil(|py| to_py_err(py, e)))?;
            Ok(s)
        })
    }

    #[pyo3(signature = (clear=true, as_exception=false))]
    pub fn get_error_state<'py>(
        &self,
        py: Python<'py>,
        clear: bool,
        as_exception: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let dev_arc = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut dev = dev_arc.lock().await;
            let (state, last_err) = dev
                .get_error_state(clear)
                .await
                .map_err(|e| Python::with_gil(|py| to_py_err(py, e)))?;
            Python::with_gil(|py| -> PyResult<(u32, PyObject)> {
                if as_exception {
                    if last_err != 0 {
                        let err = crate::protocol::errors::ShdlcError::device_error(last_err);
                        let py_err = to_py_err(py, err);
                        Ok((state, py_err.value(py).clone().unbind().into()))
                    } else {
                        Ok((state, py.None()))
                    }
                } else {
                    Ok((state, last_err.into_pyobject(py)?.unbind().into()))
                }
            })
        })
    }

    pub fn get_slave_address<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let dev_arc = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut dev = dev_arc.lock().await;
            let addr = dev
                .get_slave_address()
                .await
                .map_err(|e| Python::with_gil(|py| to_py_err(py, e)))?;
            Ok(addr)
        })
    }

    #[pyo3(signature = (slave_address, update_driver=true))]
    pub fn set_slave_address<'py>(
        &self,
        py: Python<'py>,
        slave_address: u8,
        update_driver: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let dev_arc = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut dev = dev_arc.lock().await;
            dev.set_slave_address(slave_address, update_driver)
                .await
                .map_err(|e| Python::with_gil(|py| to_py_err(py, e)))?;
            Ok(())
        })
    }

    pub fn get_baudrate<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let dev_arc = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut dev = dev_arc.lock().await;
            let baud = dev
                .get_baudrate()
                .await
                .map_err(|e| Python::with_gil(|py| to_py_err(py, e)))?;
            Ok(baud)
        })
    }

    #[pyo3(signature = (baudrate, update_driver=true))]
    pub fn set_baudrate<'py>(
        &self,
        py: Python<'py>,
        baudrate: u32,
        update_driver: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let dev_arc = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut dev = dev_arc.lock().await;
            dev.set_baudrate(baudrate, update_driver)
                .await
                .map_err(|e| Python::with_gil(|py| to_py_err(py, e)))?;
            Ok(())
        })
    }

    pub fn get_reply_delay<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let dev_arc = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut dev = dev_arc.lock().await;
            let delay = dev
                .get_reply_delay()
                .await
                .map_err(|e| Python::with_gil(|py| to_py_err(py, e)))?;
            Ok(delay)
        })
    }

    pub fn set_reply_delay<'py>(
        &self,
        py: Python<'py>,
        reply_delay: u16,
    ) -> PyResult<Bound<'py, PyAny>> {
        let dev_arc = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut dev = dev_arc.lock().await;
            dev.set_reply_delay(reply_delay)
                .await
                .map_err(|e| Python::with_gil(|py| to_py_err(py, e)))?;
            Ok(())
        })
    }

    pub fn get_system_up_time<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let dev_arc = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut dev = dev_arc.lock().await;
            let uptime = dev
                .get_system_up_time()
                .await
                .map_err(|e| Python::with_gil(|py| to_py_err(py, e)))?;
            Ok(uptime)
        })
    }

    pub fn factory_reset<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let dev_arc = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut dev = dev_arc.lock().await;
            dev.factory_reset()
                .await
                .map_err(|e| Python::with_gil(|py| to_py_err(py, e)))?;
            Ok(())
        })
    }

    pub fn device_reset<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let dev_arc = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut dev = dev_arc.lock().await;
            dev.device_reset()
                .await
                .map_err(|e| Python::with_gil(|py| to_py_err(py, e)))?;
            Ok(())
        })
    }
}
