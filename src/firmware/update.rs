use crate::device::AsyncShdlcDevice;
use crate::firmware::image::ShdlcFirmwareImage;
use crate::protocol::commands::bootloader::{
    EnterBootloader, FirmwareUpdateData, FirmwareUpdateStart, FirmwareUpdateStop,
};
use crate::protocol::errors::ShdlcError;
use std::sync::Arc;

pub const BOOTLOADER_BITRATE: u32 = 115200;
pub const BOOTLOADER_SLAVE_ADDRESS: u8 = 0;

pub type StatusCallback = Arc<dyn Fn(&str) + Send + Sync>;
pub type ProgressCallback = Arc<dyn Fn(f64) + Send + Sync>;

/// Asynchronous firmware updater for SHDLC devices.
pub struct ShdlcFirmwareUpdate {
    device: AsyncShdlcDevice,
    image: ShdlcFirmwareImage,
    status_callback: Option<StatusCallback>,
    progress_callback: Option<ProgressCallback>,
}

impl ShdlcFirmwareUpdate {
    pub fn new(device: AsyncShdlcDevice, image: ShdlcFirmwareImage) -> Self {
        Self {
            device,
            image,
            status_callback: None,
            progress_callback: None,
        }
    }

    pub fn set_status_callback<F>(&mut self, cb: F)
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.status_callback = Some(Arc::new(cb));
    }

    pub fn set_progress_callback<F>(&mut self, cb: F)
    where
        F: Fn(f64) + Send + Sync + 'static,
    {
        self.progress_callback = Some(Arc::new(cb));
    }

    fn status(&self, msg: &str) {
        if let Some(ref cb) = self.status_callback {
            cb(msg);
        }
    }

    fn progress(&self, percent: f64) {
        if let Some(ref cb) = self.progress_callback {
            cb(percent);
        }
    }

    pub async fn execute(&mut self, emergency: bool) -> Result<(), ShdlcError> {
        let transport_arc = self.device.connection().transport();
        let old_bitrate = {
            let mut transport = transport_arc.lock().await;
            let current = transport.bitrate()?;
            // Test that setting bitrate works
            transport.set_bitrate(current).await?;
            current
        };

        if !emergency {
            self.status("Check compatibility...");
            let prod_str = self.device.get_product_type().await?;
            let actual = u32::from_str_radix(&prod_str, 16).map_err(|_| {
                ShdlcError::Other(format!("Failed to parse product type hex: {}", prod_str))
            })?;
            let expected = self.image.product_type();
            if actual != expected {
                return Err(ShdlcError::FirmwareImageIncompatibilityError { expected, actual });
            }
            self.progress(4.0);

            self.status("Enter bootloader...");
            self.device.base.execute(&EnterBootloader).await?;
            self.progress(7.0);
        }

        // Switch to bootloader bitrate
        {
            let mut transport = transport_arc.lock().await;
            transport.set_bitrate(BOOTLOADER_BITRATE).await?;
        }

        let update_res = self.perform_update_sequence().await;

        // Restore original bitrate
        {
            let mut transport = transport_arc.lock().await;
            let _ = transport.set_bitrate(old_bitrate).await;
        }

        update_res?;
        self.status("Finished!");
        self.progress(100.0);
        Ok(())
    }

    async fn perform_update_sequence(&mut self) -> Result<(), ShdlcError> {
        self.status("Clear flash...");
        self.device
            .connection()
            .execute(BOOTLOADER_SLAVE_ADDRESS, &FirmwareUpdateStart, true)
            .await?;
        self.progress(10.0);

        self.status("Write new firmware...");
        self.image.reset_cursor();
        let total_size = self.image.size() as f64;
        let progress_start = 10.0;
        let progress_end = 90.0;
        let progress_diff = progress_end - progress_start;

        while self.image.available_bytes() > 0 {
            let chunk = self.image.read(Some(254));
            let cmd = FirmwareUpdateData::new(&chunk);
            self.device
                .connection()
                .execute(BOOTLOADER_SLAVE_ADDRESS, &cmd, true)
                .await?;

            let sent = total_size - self.image.available_bytes() as f64;
            let ratio = if total_size > 0.0 {
                sent / total_size
            } else {
                1.0
            };
            let percent = progress_start + (ratio * progress_diff);
            self.status(&format!(
                "Write new firmware: {:.2} kB of {:.2} kB",
                sent / 1024.0,
                total_size / 1024.0
            ));
            self.progress(percent);
        }

        self.status("Verify checksum...");
        let stop_cmd = FirmwareUpdateStop::new(self.image.checksum());
        self.device
            .connection()
            .execute(BOOTLOADER_SLAVE_ADDRESS, &stop_cmd, true)
            .await?;

        Ok(())
    }
}
