use crate::connection::AsyncShdlcConnection;
use crate::protocol::commands::ShdlcCommand;
use crate::protocol::errors::ShdlcError;
use crate::transport::traits::ShdlcTransport;
use std::sync::Arc;
use std::time::Duration;

/// Synchronous blocking SHDLC Connection for non-async applications.
#[derive(Clone)]
pub struct ShdlcConnection {
    async_conn: AsyncShdlcConnection,
    runtime: Arc<tokio::runtime::Runtime>,
}

impl ShdlcConnection {
    pub fn new(transport: Box<dyn ShdlcTransport>) -> Result<Self, ShdlcError> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| ShdlcError::Other(format!("Failed to build Tokio runtime: {}", e)))?;
        Ok(Self {
            async_conn: AsyncShdlcConnection::new(transport),
            runtime: Arc::new(rt),
        })
    }

    pub fn async_conn(&self) -> &AsyncShdlcConnection {
        &self.async_conn
    }

    pub fn transceive(
        &self,
        slave_address: u8,
        command_id: u8,
        data: &[u8],
        response_timeout: Duration,
    ) -> Result<(Vec<u8>, bool), ShdlcError> {
        self.runtime.block_on(self.async_conn.transceive(
            slave_address,
            command_id,
            data,
            response_timeout,
        ))
    }

    pub fn execute<C: ShdlcCommand>(
        &self,
        slave_address: u8,
        command: &C,
        wait_post_process: bool,
    ) -> Result<(C::Response, bool), ShdlcError> {
        self.runtime.block_on(
            self.async_conn
                .execute(slave_address, command, wait_post_process),
        )
    }
}
