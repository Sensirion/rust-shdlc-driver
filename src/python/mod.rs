pub mod commands;
pub mod connection;
pub mod device;
pub mod errors;
pub mod firmware;
pub mod frame;
pub mod ports;
pub mod types;

use pyo3::prelude::*;

pub fn register_python_module(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register exceptions
    errors::register_exceptions(py, m)?;

    // Register types
    m.add_class::<types::PyFirmwareVersion>()?;
    m.add_class::<types::PyHardwareVersion>()?;
    m.add_class::<types::PyProtocolVersion>()?;
    m.add_class::<types::PyVersion>()?;

    // Register frame builders
    m.add_class::<frame::PyShdlcSerialMosiFrameBuilder>()?;
    m.add_class::<frame::PyShdlcSerialMisoFrameBuilder>()?;

    // Register ports
    #[cfg(feature = "serial")]
    m.add_class::<ports::PyShdlcSerialPort>()?;
    m.add_class::<ports::PyShdlcTcpPort>()?;
    m.add_class::<ports::PyShdlcMockPort>()?;

    // Register connection
    m.add_class::<connection::PyShdlcConnection>()?;
    m.add_class::<connection::PyAsyncShdlcConnection>()?;

    // Register device
    m.add_class::<device::PyShdlcDeviceBase>()?;
    m.add_class::<device::PyShdlcDevice>()?;
    m.add_class::<device::PyAsyncShdlcDevice>()?;

    // Register commands
    m.add_class::<commands::PyShdlcCommand>()?;
    m.add_class::<commands::PyShdlcCmdGetProductType>()?;
    m.add_class::<commands::PyShdlcCmdGetProductName>()?;
    m.add_class::<commands::PyShdlcCmdGetArticleCode>()?;
    m.add_class::<commands::PyShdlcCmdGetSerialNumber>()?;
    m.add_class::<commands::PyShdlcCmdGetProductSubType>()?;
    m.add_class::<commands::PyShdlcCmdGetVersion>()?;
    m.add_class::<commands::PyShdlcCmdGetErrorState>()?;
    m.add_class::<commands::PyShdlcCmdDeviceReset>()?;
    m.add_class::<commands::PyShdlcCmdGetSlaveAddress>()?;
    m.add_class::<commands::PyShdlcCmdSetSlaveAddress>()?;
    m.add_class::<commands::PyShdlcCmdGetBaudrate>()?;
    m.add_class::<commands::PyShdlcCmdSetBaudrate>()?;
    m.add_class::<commands::PyShdlcCmdGetReplyDelay>()?;
    m.add_class::<commands::PyShdlcCmdSetReplyDelay>()?;
    m.add_class::<commands::PyShdlcCmdGetSystemUpTime>()?;
    m.add_class::<commands::PyShdlcCmdFactoryReset>()?;
    m.add_class::<commands::PyShdlcCmdEnterBootloader>()?;
    m.add_class::<commands::PyShdlcCmdFirmwareUpdateStart>()?;
    m.add_class::<commands::PyShdlcCmdFirmwareUpdateData>()?;
    m.add_class::<commands::PyShdlcCmdFirmwareUpdateStop>()?;

    // Register firmware
    m.add_class::<firmware::PyShdlcFirmwareImage>()?;
    m.add_class::<firmware::PyShdlcFirmwareUpdate>()?;
    m.add_class::<firmware::PyAsyncShdlcFirmwareUpdate>()?;

    // Module doc and version
    m.add("__version__", "0.1.0")?;
    m.add("__copyright__", "(c) Copyright Sensirion AG")?;

    Ok(())
}
