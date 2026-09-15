use crate::protocol::errors::ShdlcError;
use crate::protocol::frame::{ShdlcMisoFrame, ShdlcMisoFrameBuilder, ShdlcMosiFrame};
use crate::transport::traits::ShdlcTransport;
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// Transceiver logic that handles sending MOSI frames, reading bytes from an `ShdlcTransport`,
/// managing timeouts, and parsing the MISO frame response.
pub struct ShdlcTransceiver;

impl ShdlcTransceiver {
    /// Calculate the time required for receiving the longest possible frame.
    pub fn calculate_maximum_frame_time(bitrate: u32) -> Duration {
        let max_sec = (600.0 * 10.0) / (bitrate as f64) + 0.2;
        Duration::from_secs_f64(max_sec)
    }

    /// Transceive an SHDLC frame across any `ShdlcTransport` implementation.
    pub async fn transceive<T: ShdlcTransport + ?Sized>(
        transport: &mut T,
        slave_address: u8,
        command_id: u8,
        data: &[u8],
        response_timeout: Duration,
        additional_response_time: Duration,
    ) -> Result<ShdlcMisoFrame, ShdlcError> {
        // Build MOSI frame and write to transport
        let mosi = ShdlcMosiFrame::new(slave_address, command_id, data);
        let raw_tx = mosi.to_bytes();
        transport.write_all(&raw_tx).await?;
        transport.flush().await?;

        // Calculate timeouts
        let bitrate = transport.bitrate().unwrap_or(115200);
        let max_frame_time = Self::calculate_maximum_frame_time(bitrate);
        let adjusted_timeout = response_timeout + additional_response_time;
        let total_timeout = adjusted_timeout + max_frame_time;

        let start_time = Instant::now();
        let mut builder = ShdlcMisoFrameBuilder::new();
        let mut buf = [0u8; 512];

        loop {
            let elapsed = start_time.elapsed();
            let current_limit = if builder.start_received() {
                total_timeout
            } else {
                adjusted_timeout
            };

            if elapsed >= current_limit {
                return Err(ShdlcError::Timeout);
            }

            let remaining = current_limit - elapsed;
            let read_res = tokio::time::timeout(remaining, transport.read(&mut buf)).await;

            match read_res {
                Ok(Ok(n)) => {
                    if n > 0 {
                        let is_complete = builder.add_data(&buf[..n])?;
                        if is_complete {
                            return builder.interpret_data();
                        }
                    } else {
                        // 0 bytes read, sleep shortly to allow buffer fill or yield
                        sleep(Duration::from_millis(1)).await;
                    }
                }
                Ok(Err(err)) => return Err(err),
                Err(_) => {
                    // Timeout hit
                    return Err(ShdlcError::Timeout);
                }
            }
        }
    }
}
