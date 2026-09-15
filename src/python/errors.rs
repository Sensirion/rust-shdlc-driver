use crate::protocol::errors::ShdlcError;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

create_exception!(rust_shdlc_driver, ShdlcErrorPy, PyException);
create_exception!(rust_shdlc_driver, ShdlcTimeoutErrorPy, ShdlcErrorPy);
create_exception!(rust_shdlc_driver, ShdlcResponseErrorPy, ShdlcErrorPy);
create_exception!(rust_shdlc_driver, ShdlcDeviceErrorPy, ShdlcErrorPy);
create_exception!(
    rust_shdlc_driver,
    ShdlcCommandDataSizeErrorPy,
    ShdlcDeviceErrorPy
);
create_exception!(
    rust_shdlc_driver,
    ShdlcUnknownCommandErrorPy,
    ShdlcDeviceErrorPy
);
create_exception!(
    rust_shdlc_driver,
    ShdlcAccessRightErrorPy,
    ShdlcDeviceErrorPy
);
create_exception!(
    rust_shdlc_driver,
    ShdlcCommandParameterErrorPy,
    ShdlcDeviceErrorPy
);
create_exception!(rust_shdlc_driver, ShdlcChecksumErrorPy, ShdlcDeviceErrorPy);
create_exception!(
    rust_shdlc_driver,
    ShdlcFirmwareUpdateErrorPy,
    ShdlcDeviceErrorPy
);
create_exception!(
    rust_shdlc_driver,
    ShdlcFirmwareImageSignatureErrorPy,
    ShdlcErrorPy
);
create_exception!(
    rust_shdlc_driver,
    ShdlcFirmwareImageIncompatibilityErrorPy,
    ShdlcErrorPy
);

pub fn to_py_err(py: Python<'_>, err: ShdlcError) -> PyErr {
    match err {
        ShdlcError::Timeout => ShdlcTimeoutErrorPy::new_err(
            "Timeout while waiting for response from SHDLC device. Check connection to device and make sure it is powered on."
        ),
        ShdlcError::ResponseError { message, raw_data } => {
            let py_err = ShdlcResponseErrorPy::new_err(format!("Invalid data received from the SHDLC device: {}", message));
            if let Some(data) = raw_data {
                let py_bytes = PyBytes::new(py, &data);
                let _ = py_err.value(py).setattr("received_data", py_bytes);
            }
            py_err
        }
        ShdlcError::DeviceError { code, message } => {
            let full_msg = format!("SHDLC device returned error code {}: {}", code, message);
            let py_err = match code {
                1 => ShdlcCommandDataSizeErrorPy::new_err(full_msg),
                2 => ShdlcUnknownCommandErrorPy::new_err(full_msg),
                3 => ShdlcAccessRightErrorPy::new_err(full_msg),
                4 => ShdlcCommandParameterErrorPy::new_err(full_msg),
                5 => ShdlcChecksumErrorPy::new_err(full_msg),
                6 => ShdlcFirmwareUpdateErrorPy::new_err(full_msg),
                _ => ShdlcDeviceErrorPy::new_err(full_msg),
            };
            let _ = py_err.value(py).setattr("error_code", code);
            let _ = py_err.value(py).setattr("error_message", message);
            py_err
        }
        ShdlcError::FirmwareImageSignatureError(sig) => {
            let hex = sig.iter().map(|b| format!("{:02X}", b)).collect::<String>();
            ShdlcFirmwareImageSignatureErrorPy::new_err(format!("Invalid signature in firmware image: 0x{}", hex))
        }
        ShdlcError::FirmwareImageIncompatibilityError { expected, actual } => {
            ShdlcFirmwareImageIncompatibilityErrorPy::new_err(format!(
                "Firmware image for device 0x{:08X} not compatible with connected device 0x{:08X}.",
                expected, actual
            ))
        }
        ShdlcError::Transport(msg) | ShdlcError::Io(msg) | ShdlcError::PortError(msg) | ShdlcError::Other(msg) => {
            ShdlcErrorPy::new_err(msg)
        }
    }
}

pub fn register_exceptions(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("ShdlcError", m.py().get_type::<ShdlcErrorPy>())?;
    m.add(
        "ShdlcTimeoutError",
        m.py().get_type::<ShdlcTimeoutErrorPy>(),
    )?;
    m.add(
        "ShdlcResponseError",
        m.py().get_type::<ShdlcResponseErrorPy>(),
    )?;
    m.add("ShdlcDeviceError", m.py().get_type::<ShdlcDeviceErrorPy>())?;
    m.add(
        "ShdlcCommandDataSizeError",
        m.py().get_type::<ShdlcCommandDataSizeErrorPy>(),
    )?;
    m.add(
        "ShdlcUnknownCommandError",
        m.py().get_type::<ShdlcUnknownCommandErrorPy>(),
    )?;
    m.add(
        "ShdlcAccessRightError",
        m.py().get_type::<ShdlcAccessRightErrorPy>(),
    )?;
    m.add(
        "ShdlcCommandParameterError",
        m.py().get_type::<ShdlcCommandParameterErrorPy>(),
    )?;
    m.add(
        "ShdlcChecksumError",
        m.py().get_type::<ShdlcChecksumErrorPy>(),
    )?;
    m.add(
        "ShdlcFirmwareUpdateError",
        m.py().get_type::<ShdlcFirmwareUpdateErrorPy>(),
    )?;
    m.add(
        "ShdlcFirmwareImageSignatureError",
        m.py().get_type::<ShdlcFirmwareImageSignatureErrorPy>(),
    )?;
    m.add(
        "ShdlcFirmwareImageIncompatibilityError",
        m.py()
            .get_type::<ShdlcFirmwareImageIncompatibilityErrorPy>(),
    )?;
    Ok(())
}
