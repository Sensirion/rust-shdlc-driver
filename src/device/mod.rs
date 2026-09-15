pub mod sync_dev;

pub use async_dev::*;
pub use sync_dev::ShdlcDevice;

mod async_dev {
    use crate::connection::AsyncShdlcConnection;
    use crate::protocol::commands::*;
    use crate::protocol::errors::ShdlcError;
    use crate::protocol::types::Version;
    use std::collections::HashMap;

    /// Base SHDLC Device.
    #[derive(Clone)]
    pub struct AsyncShdlcDeviceBase {
        pub connection: AsyncShdlcConnection,
        pub slave_address: u8,
        pub last_error_flag: bool,
        pub registered_errors: HashMap<u8, String>,
    }

    impl AsyncShdlcDeviceBase {
        pub fn new(connection: AsyncShdlcConnection, slave_address: u8) -> Self {
            Self {
                connection,
                slave_address,
                last_error_flag: false,
                registered_errors: HashMap::new(),
            }
        }

        pub fn register_device_error(&mut self, code: u8, message: impl Into<String>) {
            self.registered_errors.insert(code, message.into());
        }

        pub async fn execute<C: ShdlcCommand>(
            &mut self,
            command: &C,
        ) -> Result<C::Response, ShdlcError> {
            match self
                .connection
                .execute(self.slave_address, command, true)
                .await
            {
                Ok((data, err_flag)) => {
                    self.last_error_flag = err_flag;
                    Ok(data)
                }
                Err(ShdlcError::DeviceError { code, message }) => {
                    let msg = self
                        .registered_errors
                        .get(&code)
                        .cloned()
                        .unwrap_or(message);
                    Err(ShdlcError::DeviceError { code, message: msg })
                }
                Err(e) => Err(e),
            }
        }
    }

    /// Generic Async SHDLC Device providing common SHDLC commands.
    #[derive(Clone)]
    pub struct AsyncShdlcDevice {
        pub base: AsyncShdlcDeviceBase,
    }

    impl AsyncShdlcDevice {
        pub fn new(connection: AsyncShdlcConnection, slave_address: u8) -> Self {
            Self {
                base: AsyncShdlcDeviceBase::new(connection, slave_address),
            }
        }

        pub fn slave_address(&self) -> u8 {
            self.base.slave_address
        }

        pub fn last_error_flag(&self) -> bool {
            self.base.last_error_flag
        }

        pub fn connection(&self) -> &AsyncShdlcConnection {
            &self.base.connection
        }

        pub async fn get_product_type(&mut self) -> Result<String, ShdlcError> {
            self.base.execute(&GetProductType).await
        }

        pub async fn get_product_subtype(&mut self) -> Result<u8, ShdlcError> {
            self.base.execute(&GetProductSubType).await
        }

        pub async fn get_product_name(&mut self) -> Result<String, ShdlcError> {
            self.base.execute(&GetProductName).await
        }

        pub async fn get_article_code(&mut self) -> Result<String, ShdlcError> {
            self.base.execute(&GetArticleCode).await
        }

        pub async fn get_serial_number(&mut self) -> Result<String, ShdlcError> {
            self.base.execute(&GetSerialNumber).await
        }

        pub async fn get_version(&mut self) -> Result<Version, ShdlcError> {
            self.base.execute(&GetVersion).await
        }

        pub async fn get_error_state(&mut self, clear: bool) -> Result<(u32, u8), ShdlcError> {
            self.base.execute(&GetErrorState::new(clear)).await
        }

        pub async fn get_slave_address(&mut self) -> Result<u8, ShdlcError> {
            self.base.execute(&GetSlaveAddress).await
        }

        pub async fn set_slave_address(
            &mut self,
            slave_address: u8,
            update_driver: bool,
        ) -> Result<(), ShdlcError> {
            self.base
                .execute(&SetSlaveAddress::new(slave_address))
                .await?;
            if update_driver {
                self.base.slave_address = slave_address;
            }
            Ok(())
        }

        pub async fn get_baudrate(&mut self) -> Result<u32, ShdlcError> {
            self.base.execute(&GetBaudrate).await
        }

        pub async fn set_baudrate(
            &mut self,
            baudrate: u32,
            update_driver: bool,
        ) -> Result<(), ShdlcError> {
            self.base.execute(&SetBaudrate::new(baudrate)).await?;
            if update_driver {
                let transport_arc = self.base.connection.transport();
                let mut transport = transport_arc.lock().await;
                let _ = transport.set_bitrate(baudrate).await;
            }
            Ok(())
        }

        pub async fn get_reply_delay(&mut self) -> Result<u16, ShdlcError> {
            self.base.execute(&GetReplyDelay).await
        }

        pub async fn set_reply_delay(&mut self, reply_delay_us: u16) -> Result<(), ShdlcError> {
            self.base.execute(&SetReplyDelay::new(reply_delay_us)).await
        }

        pub async fn get_system_up_time(&mut self) -> Result<u32, ShdlcError> {
            self.base.execute(&GetSystemUpTime).await
        }

        pub async fn device_reset(&mut self) -> Result<(), ShdlcError> {
            self.base.execute(&DeviceReset).await
        }

        pub async fn factory_reset(&mut self) -> Result<(), ShdlcError> {
            self.base.execute(&FactoryReset).await
        }

        pub async fn execute_raw(
            &mut self,
            command: &RawShdlcCommand,
        ) -> Result<Vec<u8>, ShdlcError> {
            self.base.execute(command).await
        }
    }
}
