use crate::protocol::errors::ShdlcError;
use async_trait::async_trait;

/// Asynchronous transport interface for transceiving raw bytes on an SHDLC bus (e.g. Serial, TCP, Mock).
#[async_trait]
pub trait ShdlcTransport: Send + Sync {
    /// Send raw bytes to the transport.
    async fn write_all(&mut self, data: &[u8]) -> Result<(), ShdlcError>;

    /// Read available bytes into the provided buffer.
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, ShdlcError>;

    /// Flush any pending write buffers.
    async fn flush(&mut self) -> Result<(), ShdlcError>;

    /// Set the baudrate / bitrate of the transport in bit/s.
    async fn set_bitrate(&mut self, bitrate: u32) -> Result<(), ShdlcError>;

    /// Get the current baudrate / bitrate of the transport in bit/s.
    fn bitrate(&self) -> Result<u32, ShdlcError>;

    /// Get a textual description of the port (e.g. "/dev/ttyUSB0@115200" or "192.168.1.1:8000").
    fn description(&self) -> String;

    /// Check if the transport is open.
    fn is_open(&self) -> bool;

    /// Close the transport and release underlying resources.
    async fn close(&mut self) -> Result<(), ShdlcError>;
}
