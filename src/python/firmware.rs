use crate::firmware::image::ShdlcFirmwareImage;
use crate::firmware::update::ShdlcFirmwareUpdate;
use crate::python::device::{PyAsyncShdlcDevice, PyShdlcDevice};
use crate::python::errors::to_py_err;
use crate::python::types::PyFirmwareVersion;
use crate::transport::runtime::get_runtime;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::fs;

#[pyclass(name = "ShdlcFirmwareImage")]
#[derive(Clone)]
pub struct PyShdlcFirmwareImage {
    pub inner: ShdlcFirmwareImage,
}

#[pymethods]
impl PyShdlcFirmwareImage {
    #[new]
    #[pyo3(signature = (hexfile, bl_start_addr, app_start_addr, signature=None, bl_version_offset=None))]
    pub fn new(
        py: Python<'_>,
        hexfile: Bound<'_, PyAny>,
        bl_start_addr: u32,
        app_start_addr: u32,
        signature: Option<&[u8]>,
        bl_version_offset: Option<u32>,
    ) -> PyResult<Self> {
        let content: String = if let Ok(s) = hexfile.extract::<String>() {
            // Check if file path
            if fs::metadata(&s).is_ok() {
                fs::read_to_string(&s).map_err(|e| {
                    to_py_err(py, crate::protocol::errors::ShdlcError::Io(e.to_string()))
                })?
            } else {
                s
            }
        } else if hexfile.hasattr("read")? {
            // File-like object
            let res = hexfile.call_method0("read")?;
            if let Ok(s) = res.extract::<String>() {
                s
            } else if let Ok(b) = res.extract::<Vec<u8>>() {
                String::from_utf8_lossy(&b).to_string()
            } else {
                return Err(to_py_err(
                    py,
                    crate::protocol::errors::ShdlcError::Other(
                        "Cannot read from file-like object".to_string(),
                    ),
                ));
            }
        } else {
            return Err(to_py_err(
                py,
                crate::protocol::errors::ShdlcError::Other("Invalid hexfile argument".to_string()),
            ));
        };

        let image = ShdlcFirmwareImage::new(
            &content,
            bl_start_addr,
            app_start_addr,
            signature,
            bl_version_offset,
        )
        .map_err(|e| to_py_err(py, e))?;

        Ok(Self { inner: image })
    }

    #[getter]
    pub fn product_type(&self) -> u32 {
        self.inner.product_type()
    }

    #[getter]
    pub fn bootloader_version(&self) -> PyFirmwareVersion {
        self.inner.bootloader_version().into()
    }

    #[getter]
    pub fn application_version(&self) -> PyFirmwareVersion {
        self.inner.application_version().into()
    }

    #[getter]
    pub fn checksum(&self) -> u8 {
        self.inner.checksum()
    }

    #[getter]
    pub fn size(&self) -> usize {
        self.inner.size()
    }

    #[getter]
    pub fn available_bytes(&self) -> usize {
        self.inner.available_bytes()
    }

    #[pyo3(signature = (size=-1))]
    pub fn read<'py>(&mut self, py: Python<'py>, size: isize) -> Bound<'py, PyBytes> {
        let opt_size = if size < 0 { None } else { Some(size as usize) };
        let chunk = self.inner.read(opt_size);
        PyBytes::new(py, &chunk)
    }
}

// ---------------------------------------------------------------------------
// Sync ShdlcFirmwareUpdate
// ---------------------------------------------------------------------------
#[pyclass(name = "ShdlcFirmwareUpdate")]
pub struct PyShdlcFirmwareUpdate {
    device: Py<PyShdlcDevice>,
    image: PyShdlcFirmwareImage,
    status_cb: Option<PyObject>,
    progress_cb: Option<PyObject>,
}

#[pymethods]
impl PyShdlcFirmwareUpdate {
    #[new]
    #[pyo3(signature = (device, image, status_callback=None, progress_callback=None))]
    pub fn new(
        _py: Python<'_>,
        device: Py<PyShdlcDevice>,
        image: PyShdlcFirmwareImage,
        status_callback: Option<PyObject>,
        progress_callback: Option<PyObject>,
    ) -> Self {
        Self {
            device,
            image,
            status_cb: status_callback,
            progress_cb: progress_callback,
        }
    }

    #[pyo3(signature = (emergency=false))]
    pub fn execute(&mut self, py: Python<'_>, emergency: bool) -> PyResult<()> {
        let bound_dev = self.device.bind(py);
        let base_ref = bound_dev.borrow();
        let base = base_ref.as_ref();
        let async_dev = crate::device::AsyncShdlcDevice::new(
            base.conn_obj.async_conn.clone(),
            base.slave_address,
        );

        let mut updater = ShdlcFirmwareUpdate::new(async_dev, self.image.inner.clone());

        if let Some(ref cb) = self.status_cb {
            let cb_clone = cb.clone_ref(py);
            updater.set_status_callback(move |status| {
                Python::with_gil(|py| {
                    let _ = cb_clone.call1(py, (status,));
                });
            });
        }

        if let Some(ref cb) = self.progress_cb {
            let cb_clone = cb.clone_ref(py);
            updater.set_progress_callback(move |progress| {
                Python::with_gil(|py| {
                    let _ = cb_clone.call1(py, (progress,));
                });
            });
        }

        get_runtime()
            .block_on(updater.execute(emergency))
            .map_err(|e| to_py_err(py, e))
    }
}

// ---------------------------------------------------------------------------
// Async ShdlcFirmwareUpdate
// ---------------------------------------------------------------------------
#[pyclass(name = "AsyncShdlcFirmwareUpdate")]
pub struct PyAsyncShdlcFirmwareUpdate {
    device_obj: PyAsyncShdlcDevice,
    image: PyShdlcFirmwareImage,
    status_cb: Option<PyObject>,
    progress_cb: Option<PyObject>,
}

#[pymethods]
impl PyAsyncShdlcFirmwareUpdate {
    #[new]
    #[pyo3(signature = (device, image, status_callback=None, progress_callback=None))]
    pub fn new(
        device: PyAsyncShdlcDevice,
        image: PyShdlcFirmwareImage,
        status_callback: Option<PyObject>,
        progress_callback: Option<PyObject>,
    ) -> Self {
        Self {
            device_obj: device,
            image,
            status_cb: status_callback,
            progress_cb: progress_callback,
        }
    }

    #[pyo3(signature = (emergency=false))]
    pub fn execute<'py>(&self, py: Python<'py>, emergency: bool) -> PyResult<Bound<'py, PyAny>> {
        let dev_arc = self.device_obj.inner.clone();
        let image = self.image.inner.clone();
        let status_cb = self.status_cb.as_ref().map(|cb| cb.clone_ref(py));
        let progress_cb = self.progress_cb.as_ref().map(|cb| cb.clone_ref(py));

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let dev_guard = dev_arc.lock().await;
            let mut updater = ShdlcFirmwareUpdate::new(dev_guard.clone(), image);

            if let Some(cb) = status_cb {
                updater.set_status_callback(move |status| {
                    Python::with_gil(|py| {
                        let _ = cb.call1(py, (status,));
                    });
                });
            }

            if let Some(cb) = progress_cb {
                updater.set_progress_callback(move |progress| {
                    Python::with_gil(|py| {
                        let _ = cb.call1(py, (progress,));
                    });
                });
            }

            updater
                .execute(emergency)
                .await
                .map_err(|e| Python::with_gil(|py| to_py_err(py, e)))?;

            Ok(())
        })
    }
}
