use crate::connection::ShdlcConnection;
use crate::device::AsyncShdlcDevice;
use crate::protocol::commands::RawShdlcCommand;
use crate::protocol::errors::ShdlcError;
use crate::protocol::types::Version;

/// Synchronous blocking SHDLC device.
#[derive(Clone)]
pub struct ShdlcDevice {
    async_dev: AsyncShdlcDevice,
    conn: ShdlcConnection,
}

impl ShdlcDevice {
    pub fn new(conn: ShdlcConnection, slave_address: u8) -> Self {
        let async_dev = AsyncShdlcDevice::new(conn.async_conn().clone(), slave_address);
        Self { async_dev, conn }
    }

    pub fn slave_address(&self) -> u8 {
        self.async_dev.slave_address()
    }

    pub fn last_error_flag(&self) -> bool {
        self.async_dev.last_error_flag()
    }

    pub fn get_product_type(&mut self) -> Result<String, ShdlcError> {
        self.conn
            .execute(
                self.async_dev.slave_address(),
                &crate::protocol::commands::GetProductType,
                true,
            )
            .map(|r| r.0)
    }

    pub fn get_product_subtype(&mut self) -> Result<u8, ShdlcError> {
        self.conn
            .execute(
                self.async_dev.slave_address(),
                &crate::protocol::commands::GetProductSubType,
                true,
            )
            .map(|r| r.0)
    }

    pub fn get_product_name(&mut self) -> Result<String, ShdlcError> {
        self.conn
            .execute(
                self.async_dev.slave_address(),
                &crate::protocol::commands::GetProductName,
                true,
            )
            .map(|r| r.0)
    }

    pub fn get_article_code(&mut self) -> Result<String, ShdlcError> {
        self.conn
            .execute(
                self.async_dev.slave_address(),
                &crate::protocol::commands::GetArticleCode,
                true,
            )
            .map(|r| r.0)
    }

    pub fn get_serial_number(&mut self) -> Result<String, ShdlcError> {
        self.conn
            .execute(
                self.async_dev.slave_address(),
                &crate::protocol::commands::GetSerialNumber,
                true,
            )
            .map(|r| r.0)
    }

    pub fn get_version(&mut self) -> Result<Version, ShdlcError> {
        self.conn
            .execute(
                self.async_dev.slave_address(),
                &crate::protocol::commands::GetVersion,
                true,
            )
            .map(|r| r.0)
    }

    pub fn get_error_state(&mut self, clear: bool) -> Result<(u32, u8), ShdlcError> {
        self.conn
            .execute(
                self.async_dev.slave_address(),
                &crate::protocol::commands::GetErrorState::new(clear),
                true,
            )
            .map(|r| r.0)
    }

    pub fn get_slave_address(&mut self) -> Result<u8, ShdlcError> {
        self.conn
            .execute(
                self.async_dev.slave_address(),
                &crate::protocol::commands::GetSlaveAddress,
                true,
            )
            .map(|r| r.0)
    }

    pub fn set_slave_address(
        &mut self,
        slave_address: u8,
        update_driver: bool,
    ) -> Result<(), ShdlcError> {
        self.conn.execute(
            self.async_dev.slave_address(),
            &crate::protocol::commands::SetSlaveAddress::new(slave_address),
            true,
        )?;
        if update_driver {
            self.async_dev.base.slave_address = slave_address;
        }
        Ok(())
    }

    pub fn get_baudrate(&mut self) -> Result<u32, ShdlcError> {
        self.conn
            .execute(
                self.async_dev.slave_address(),
                &crate::protocol::commands::GetBaudrate,
                true,
            )
            .map(|r| r.0)
    }

    pub fn set_baudrate(&mut self, baudrate: u32, update_driver: bool) -> Result<(), ShdlcError> {
        self.conn.execute(
            self.async_dev.slave_address(),
            &crate::protocol::commands::SetBaudrate::new(baudrate),
            true,
        )?;
        if update_driver {
            let transport_arc = self.conn.async_conn().transport();
            let _ = crate::transport::runtime::get_runtime().block_on(async move {
                let mut transport = transport_arc.lock().await;
                transport.set_bitrate(baudrate).await
            });
        }
        Ok(())
    }

    pub fn get_reply_delay(&mut self) -> Result<u16, ShdlcError> {
        self.conn
            .execute(
                self.async_dev.slave_address(),
                &crate::protocol::commands::GetReplyDelay,
                true,
            )
            .map(|r| r.0)
    }

    pub fn set_reply_delay(&mut self, reply_delay_us: u16) -> Result<(), ShdlcError> {
        self.conn
            .execute(
                self.async_dev.slave_address(),
                &crate::protocol::commands::SetReplyDelay::new(reply_delay_us),
                true,
            )
            .map(|r| r.0)
    }

    pub fn get_system_up_time(&mut self) -> Result<u32, ShdlcError> {
        self.conn
            .execute(
                self.async_dev.slave_address(),
                &crate::protocol::commands::GetSystemUpTime,
                true,
            )
            .map(|r| r.0)
    }

    pub fn device_reset(&mut self) -> Result<(), ShdlcError> {
        self.conn
            .execute(
                self.async_dev.slave_address(),
                &crate::protocol::commands::DeviceReset,
                true,
            )
            .map(|r| r.0)
    }

    pub fn factory_reset(&mut self) -> Result<(), ShdlcError> {
        self.conn
            .execute(
                self.async_dev.slave_address(),
                &crate::protocol::commands::FactoryReset,
                true,
            )
            .map(|r| r.0)
    }

    pub fn execute_raw(&mut self, command: &RawShdlcCommand) -> Result<Vec<u8>, ShdlcError> {
        self.conn
            .execute(self.async_dev.slave_address(), command, true)
            .map(|r| r.0)
    }
}
