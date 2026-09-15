use crate::protocol::errors::ShdlcError;
use crate::transport::runtime::enter_runtime_context;
use crate::transport::traits::ShdlcTransport;
#[cfg(feature = "serial")]
use async_trait::async_trait;
#[cfg(feature = "serial")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "serial")]
use tokio_serial::{SerialPort, SerialPortBuilderExt, SerialStream};

#[cfg(feature = "serial")]
pub struct AsyncSerialPort {
    port_name: String,
    baudrate: u32,
    stream: Option<SerialStream>,
}

#[cfg(feature = "serial")]
impl AsyncSerialPort {
    pub fn new(port_name: impl Into<String>, baudrate: u32) -> Self {
        Self {
            port_name: port_name.into(),
            baudrate,
            stream: None,
        }
    }

    pub fn open(&mut self) -> Result<(), ShdlcError> {
        if self.stream.is_none() {
            let port_name = self.port_name.clone();
            let baudrate = self.baudrate;

            let res = enter_runtime_context(|| {
                tokio_serial::new(&port_name, baudrate)
                    .data_bits(tokio_serial::DataBits::Eight)
                    .flow_control(tokio_serial::FlowControl::None)
                    .parity(tokio_serial::Parity::None)
                    .stop_bits(tokio_serial::StopBits::One)
                    .open_native_async()
            });

            let stream = res.map_err(|e| {
                ShdlcError::PortError(format!(
                    "Failed to open serial port {}: {}",
                    self.port_name, e
                ))
            })?;
            self.stream = Some(stream);
        }
        Ok(())
    }
}

#[cfg(feature = "serial")]
#[async_trait]
impl ShdlcTransport for AsyncSerialPort {
    async fn write_all(&mut self, data: &[u8]) -> Result<(), ShdlcError> {
        let stream = self.stream.as_mut().ok_or_else(|| {
            ShdlcError::PortError(format!("Serial port {} is not open", self.port_name))
        })?;
        stream
            .write_all(data)
            .await
            .map_err(|e| ShdlcError::Io(e.to_string()))?;
        Ok(())
    }

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, ShdlcError> {
        let stream = self.stream.as_mut().ok_or_else(|| {
            ShdlcError::PortError(format!("Serial port {} is not open", self.port_name))
        })?;
        let n = stream
            .read(buf)
            .await
            .map_err(|e| ShdlcError::Io(e.to_string()))?;
        Ok(n)
    }

    async fn flush(&mut self) -> Result<(), ShdlcError> {
        let stream = self.stream.as_mut().ok_or_else(|| {
            ShdlcError::PortError(format!("Serial port {} is not open", self.port_name))
        })?;
        stream
            .flush()
            .await
            .map_err(|e| ShdlcError::Io(e.to_string()))?;
        Ok(())
    }

    async fn set_bitrate(&mut self, bitrate: u32) -> Result<(), ShdlcError> {
        if let Some(ref mut stream) = self.stream {
            stream.set_baud_rate(bitrate).map_err(|e| {
                ShdlcError::PortError(format!("Failed to set baudrate to {}: {}", bitrate, e))
            })?;
        }
        self.baudrate = bitrate;
        Ok(())
    }

    fn bitrate(&self) -> Result<u32, ShdlcError> {
        Ok(self.baudrate)
    }

    fn description(&self) -> String {
        format!("{}@{}", self.port_name, self.baudrate)
    }

    fn is_open(&self) -> bool {
        self.stream.is_some()
    }

    async fn close(&mut self) -> Result<(), ShdlcError> {
        self.stream = None;
        Ok(())
    }
}
