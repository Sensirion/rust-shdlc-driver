use crate::protocol::errors::ShdlcError;
use crate::transport::traits::ShdlcTransport;
use async_trait::async_trait;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// A flexible in-memory mock transport for testing without physical hardware.
#[derive(Debug, Clone)]
pub struct MockTransport {
    inner: Arc<Mutex<MockInner>>,
}

#[derive(Debug)]
struct MockInner {
    description: String,
    bitrate: u32,
    is_open: bool,
    read_queue: VecDeque<Vec<u8>>,
    written_data: Vec<Vec<u8>>,
    read_delay: Option<Duration>,
    timeout_on_read: bool,
    support_bitrate_change: bool,
}

impl MockTransport {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(MockInner {
                description: "MockTransport@115200".to_string(),
                bitrate: 115200,
                is_open: true,
                read_queue: VecDeque::new(),
                written_data: Vec::new(),
                read_delay: None,
                timeout_on_read: false,
                support_bitrate_change: true,
            })),
        }
    }

    pub fn with_bitrate(bitrate: u32) -> Self {
        let t = Self::new();
        t.inner.lock().unwrap().bitrate = bitrate;
        t
    }

    /// Enqueue raw bytes or complete frames to be read on next `read()` calls.
    pub fn push_rx_data(&self, data: &[u8]) {
        self.inner
            .lock()
            .unwrap()
            .read_queue
            .push_back(data.to_vec());
    }

    /// Retrieve all data slices that were written to this transport.
    pub fn get_written_data(&self) -> Vec<Vec<u8>> {
        self.inner.lock().unwrap().written_data.clone()
    }

    /// Clear recorded writes.
    pub fn clear_written_data(&self) {
        self.inner.lock().unwrap().written_data.clear();
    }

    /// Configure simulated read delay.
    pub fn set_read_delay(&self, delay: Option<Duration>) {
        self.inner.lock().unwrap().read_delay = delay;
    }

    /// Configure mock to timeout when reading.
    pub fn set_timeout_on_read(&self, timeout: bool) {
        self.inner.lock().unwrap().timeout_on_read = timeout;
    }

    /// Set whether changing bitrate is supported.
    pub fn set_support_bitrate_change(&self, support: bool) {
        self.inner.lock().unwrap().support_bitrate_change = support;
    }
}

impl Default for MockTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ShdlcTransport for MockTransport {
    async fn write_all(&mut self, data: &[u8]) -> Result<(), ShdlcError> {
        let mut inner = self.inner.lock().unwrap();
        if !inner.is_open {
            return Err(ShdlcError::Transport("Port is closed".to_string()));
        }
        inner.written_data.push(data.to_vec());
        Ok(())
    }

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, ShdlcError> {
        let (delay, timeout, is_open) = {
            let inner = self.inner.lock().unwrap();
            (inner.read_delay, inner.timeout_on_read, inner.is_open)
        };

        if !is_open {
            return Err(ShdlcError::Transport("Port is closed".to_string()));
        }

        if let Some(d) = delay {
            tokio::time::sleep(d).await;
        }

        if timeout {
            tokio::time::sleep(Duration::from_secs(60)).await;
            return Err(ShdlcError::Timeout);
        }

        let chunk_opt = {
            let mut inner = self.inner.lock().unwrap();
            if let Some(mut chunk) = inner.read_queue.pop_front() {
                if chunk.is_empty() {
                    Some(0)
                } else {
                    let n = std::cmp::min(buf.len(), chunk.len());
                    buf[..n].copy_from_slice(&chunk[..n]);
                    if n < chunk.len() {
                        let remaining = chunk.split_off(n);
                        inner.read_queue.push_front(remaining);
                    }
                    Some(n)
                }
            } else {
                None
            }
        };

        match chunk_opt {
            Some(n) => Ok(n),
            None => {
                tokio::task::yield_now().await;
                Ok(0)
            }
        }
    }

    async fn flush(&mut self) -> Result<(), ShdlcError> {
        Ok(())
    }

    async fn set_bitrate(&mut self, bitrate: u32) -> Result<(), ShdlcError> {
        let mut inner = self.inner.lock().unwrap();
        if !inner.support_bitrate_change {
            return Err(ShdlcError::PortError(
                "Changing bitrate is not supported on this port".to_string(),
            ));
        }
        inner.bitrate = bitrate;
        Ok(())
    }

    fn bitrate(&self) -> Result<u32, ShdlcError> {
        let inner = self.inner.lock().unwrap();
        if !inner.support_bitrate_change {
            return Err(ShdlcError::PortError(
                "Reading bitrate is not supported on this port".to_string(),
            ));
        }
        Ok(inner.bitrate)
    }

    fn description(&self) -> String {
        let inner = self.inner.lock().unwrap();
        inner.description.clone()
    }

    fn is_open(&self) -> bool {
        self.inner.lock().unwrap().is_open
    }

    async fn close(&mut self) -> Result<(), ShdlcError> {
        self.inner.lock().unwrap().is_open = false;
        Ok(())
    }
}
