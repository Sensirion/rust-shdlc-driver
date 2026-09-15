use crate::protocol::errors::ShdlcError;
use crate::transport::traits::ShdlcTransport;
use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct AsyncTcpPort {
    ip: String,
    port: u16,
    stream: Option<TcpStream>,
}

impl AsyncTcpPort {
    pub fn new(ip: impl Into<String>, port: u16) -> Self {
        Self {
            ip: ip.into(),
            port,
            stream: None,
        }
    }

    pub async fn open(&mut self) -> Result<(), ShdlcError> {
        if self.stream.is_none() {
            let addr = format!("{}:{}", self.ip, self.port);
            let stream = TcpStream::connect(&addr).await.map_err(|e| {
                ShdlcError::PortError(format!("Failed to connect to TCP {}: {}", addr, e))
            })?;
            self.stream = Some(stream);
        }
        Ok(())
    }
}

#[async_trait]
impl ShdlcTransport for AsyncTcpPort {
    async fn write_all(&mut self, data: &[u8]) -> Result<(), ShdlcError> {
        let stream = self.stream.as_mut().ok_or_else(|| {
            ShdlcError::PortError(format!("TCP socket {}:{} is not open", self.ip, self.port))
        })?;
        stream
            .write_all(data)
            .await
            .map_err(|e| ShdlcError::Io(e.to_string()))?;
        Ok(())
    }

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, ShdlcError> {
        let stream = self.stream.as_mut().ok_or_else(|| {
            ShdlcError::PortError(format!("TCP socket {}:{} is not open", self.ip, self.port))
        })?;
        let n = stream
            .read(buf)
            .await
            .map_err(|e| ShdlcError::Io(e.to_string()))?;
        if n == 0 {
            // EOF
            return Err(ShdlcError::Timeout);
        }
        Ok(n)
    }

    async fn flush(&mut self) -> Result<(), ShdlcError> {
        let stream = self.stream.as_mut().ok_or_else(|| {
            ShdlcError::PortError(format!("TCP socket {}:{} is not open", self.ip, self.port))
        })?;
        stream
            .flush()
            .await
            .map_err(|e| ShdlcError::Io(e.to_string()))?;
        Ok(())
    }

    async fn set_bitrate(&mut self, _bitrate: u32) -> Result<(), ShdlcError> {
        Err(ShdlcError::PortError(
            "The used port 'AsyncTcpPort' does not support changing the bitrate, therefore it's not possible to update the firmware or change baudrate."
                .to_string(),
        ))
    }

    fn bitrate(&self) -> Result<u32, ShdlcError> {
        // TCP has virtual bitrate
        Ok(115200)
    }

    fn description(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    fn is_open(&self) -> bool {
        self.stream.is_some()
    }

    async fn close(&mut self) -> Result<(), ShdlcError> {
        self.stream = None;
        Ok(())
    }
}
