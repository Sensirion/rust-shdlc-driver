pub mod connection;
pub mod device;
pub mod firmware;
pub mod protocol;
pub mod transport;

#[cfg(feature = "python")]
pub mod python;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn rust_shdlc_driver(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    python::register_python_module(py, m)
}
