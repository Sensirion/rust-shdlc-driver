# Sensirion SHDLC Driver (Rust + Python)

[![CI](https://github.com/Sensirion/rust-shdlc-driver/actions/workflows/ci.yml/badge.svg)](https://github.com/Sensirion/rust-shdlc-driver/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](LICENSE)

High-performance asynchronous Rust SHDLC driver and Python bindings for Sensirion SHDLC devices over serial, TCP, and custom transports.

## Features

- **Synchronous & Asynchronous APIs**: Provides both blocking (`ShdlcDevice`, `ShdlcConnection`, `ShdlcSerialPort`, `ShdlcTcpPort`, `ShdlcFirmwareUpdate`) and async (`AsyncShdlcDevice`, `AsyncShdlcConnection`, `AsyncShdlcFirmwareUpdate`) APIs in Rust and Python.
- **Python Bindings (Python >= 3.11)**: Native PyO3 and Maturin bindings with full type support and asyncio integration.
- **Firmware Update**: Complete support for Intel-Hex firmware images, signature validation, checksum verification, and device flashing over SHDLC bootloader.
- **Mock Transport**: Built-in high-fidelity in-memory mock transport for test-driven development and unit testing without physical hardware.

## Rust Usage

### Installation

Add `rust-shdlc-driver` to your project using `cargo add`:

```bash
cargo add rust-shdlc-driver
```

Or add it directly to your `Cargo.toml`:

```toml
[dependencies]
rust-shdlc-driver = "0.1.0"
```

### Basic Example (Rust)

#### Synchronous

```rust
use rust_shdlc_driver::connection::ShdlcConnection;
use rust_shdlc_driver::device::ShdlcDevice;
use rust_shdlc_driver::transport::AsyncSerialPort;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open serial port at 115200 baud
    let mut port = AsyncSerialPort::new("/dev/ttyUSB0", 115200);
    port.open()?;

    let conn = ShdlcConnection::new(Box::new(port))?;
    let mut device = ShdlcDevice::new(conn, 0);

    let product_name = device.get_product_name()?;
    let serial_number = device.get_serial_number()?;

    println!("Product: {}, Serial: {}", product_name, serial_number);
    Ok(())
}
```

#### Asynchronous (Tokio)

```rust
use rust_shdlc_driver::connection::AsyncShdlcConnection;
use rust_shdlc_driver::device::AsyncShdlcDevice;
use rust_shdlc_driver::transport::AsyncSerialPort;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut port = AsyncSerialPort::new("/dev/ttyUSB0", 115200);
    port.open()?;

    let conn = AsyncShdlcConnection::new(Box::new(port));
    let mut device = AsyncShdlcDevice::new(conn, 0);

    let product_name = device.get_product_name().await?;
    let serial_number = device.get_serial_number().await?;

    println!("Product: {}, Serial: {}", product_name, serial_number);
    Ok(())
}
```

## Python Usage

### Installation

It is strongly recommended to install the package inside a dedicated Python virtual environment (`venv`):

#### Linux / macOS

```bash
# Create and activate a virtual environment
python3 -m venv .venv
source .venv/bin/activate

# Install via pip
pip install rust-shdlc-driver
```

#### Windows

```bat
# Create and activate a virtual environment
py -m venv .venv
.venv\Scripts\activate

# Install via pip
pip install rust-shdlc-driver
```

### Basic Example (Python)

#### Synchronous

```python
from rust_shdlc_driver import ShdlcSerialPort, ShdlcConnection, ShdlcDevice

with ShdlcSerialPort(port="/dev/ttyUSB0", baudrate=115200) as port:
    conn = ShdlcConnection(port)
    device = ShdlcDevice(conn, slave_address=0)

    product_name = device.get_product_name()
    serial_number = device.get_serial_number()

    print(f"Product: {product_name}, Serial: {serial_number}")
```

#### Asynchronous (asyncio)

```python
import asyncio
from rust_shdlc_driver import ShdlcSerialPort, AsyncShdlcConnection, AsyncShdlcDevice

async def main():
    port = ShdlcSerialPort(port="/dev/ttyUSB0", baudrate=115200)
    conn = AsyncShdlcConnection(port)
    device = AsyncShdlcDevice(conn, slave_address=0)

    product_name = await device.get_product_name()
    serial_number = await device.get_serial_number()

    print(f"Product: {product_name}, Serial: {serial_number}")

asyncio.run(main())
```

## Examples

Additional standalone examples for both Rust and Python can be found in the [`examples/`](examples) directory:

- **Rust examples**:
  - `examples/sync_device.rs` – Synchronous serial communication (`cargo run --example sync_device`)
  - `examples/async_device.rs` – Asynchronous communication with Tokio (`cargo run --example async_device`)
  - `examples/mock_device.rs` – In-memory mock transport testing (`cargo run --example mock_device`)
- **Python examples**:
  - `examples/python/sync_device.py` – Synchronous serial communication
  - `examples/python/async_device.py` – Asynchronous communication with `asyncio`
  - `examples/python/mock_device.py` – In-memory mock transport testing

## Developer Guide & Building from Source

Detailed developer instructions on building the Rust and Python packages from source, running tests, and compiling documentation are available in the Sphinx documentation:

- See the [Developer Guide & Build Instructions](docs/build.rst).

To build the Sphinx documentation locally:

```bash
pip install sphinx sphinx_rtd_theme
sphinx-build -b html docs docs/_build/html
```

## License

See [LICENSE](LICENSE).
