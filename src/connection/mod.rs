pub mod sync_conn;

pub use async_connection::*;
pub use sync_conn::ShdlcConnection;

mod async_connection {
    use crate::protocol::commands::ShdlcCommand;
    use crate::protocol::errors::ShdlcError;
    use crate::transport::traits::ShdlcTransport;
    use crate::transport::transceiver::ShdlcTransceiver;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::Mutex;

    /// Async SHDLC Connection managing communication across an underlying SHDLC transport.
    #[derive(Clone)]
    pub struct AsyncShdlcConnection {
        transport: Arc<Mutex<Box<dyn ShdlcTransport>>>,
        additional_response_time: Duration,
    }

    impl AsyncShdlcConnection {
        pub fn new(transport: Box<dyn ShdlcTransport>) -> Self {
            Self {
                transport: Arc::new(Mutex::new(transport)),
                additional_response_time: Duration::from_millis(100),
            }
        }

        pub fn with_additional_response_time(mut self, time: Duration) -> Self {
            self.additional_response_time = time;
            self
        }

        pub fn transport(&self) -> Arc<Mutex<Box<dyn ShdlcTransport>>> {
            Arc::clone(&self.transport)
        }

        pub fn additional_response_time(&self) -> Duration {
            self.additional_response_time
        }

        pub fn set_additional_response_time(&mut self, time: Duration) {
            self.additional_response_time = time;
        }

        /// Transceive raw SHDLC command and validate address and command ID.
        pub async fn transceive(
            &self,
            slave_address: u8,
            command_id: u8,
            data: &[u8],
            response_timeout: Duration,
        ) -> Result<(Vec<u8>, bool), ShdlcError> {
            let mut transport_guard = self.transport.lock().await;
            let miso = ShdlcTransceiver::transceive(
                &mut **transport_guard,
                slave_address,
                command_id,
                data,
                response_timeout,
                self.additional_response_time,
            )
            .await?;

            if miso.slave_address != slave_address {
                return Err(ShdlcError::response_error(
                    format!(
                        "Received slave address {} instead of {}.",
                        miso.slave_address, slave_address
                    ),
                    Some(miso.data),
                ));
            }

            if miso.command_id != command_id {
                return Err(ShdlcError::response_error(
                    format!(
                        "Received command ID 0x{:02X} instead of 0x{:02X}.",
                        miso.command_id, command_id
                    ),
                    Some(miso.data),
                ));
            }

            let error_state = miso.is_error_state();
            let error_code = miso.error_code();

            if error_code != 0 {
                return Err(ShdlcError::device_error(error_code));
            }

            Ok((miso.data, error_state))
        }

        /// Execute a typed SHDLC command.
        pub async fn execute<C: ShdlcCommand>(
            &self,
            slave_address: u8,
            command: &C,
            wait_post_process: bool,
        ) -> Result<(C::Response, bool), ShdlcError> {
            let (raw_data, error_state) = self
                .transceive(
                    slave_address,
                    command.id(),
                    &command.data(),
                    command.max_response_time(),
                )
                .await?;

            if wait_post_process && command.post_processing_time() > Duration::from_secs(0) {
                tokio::time::sleep(command.post_processing_time()).await;
            }

            command.check_response_length(&raw_data)?;
            let interpreted = command.interpret_response(&raw_data)?;
            Ok((interpreted, error_state))
        }
    }
}
