# rust-shdlc-driver

High-performance asynchronous Rust SHDLC driver and Python bindings for Sensirion SHDLC devices over serial, TCP, and custom transports.

## Features

- **Synchronous & Asynchronous APIs**: Provides both blocking (`ShdlcDevice`, `ShdlcConnection`, `ShdlcSerialPort`, `ShdlcTcpPort`, `ShdlcFirmwareUpdate`) and async (`AsyncShdlcDevice`, `AsyncShdlcConnection`, `AsyncShdlcFirmwareUpdate`) APIs in Rust and Python.
- **Python Bindings (Python >= 3.11)**: Native PyO3 and Maturin bindings with full type support and asyncio integration.
- **Firmware Update**: Complete support for Intel-Hex firmware images, signature validation, checksum verification, and device flashing over SHDLC bootloader.
- **Mock Transport**: Built-in high-fidelity in-memory mock transport for test-driven development and unit testing without physical hardware.

## Virtual Environment Setup (Recommended)

It is strongly recommended to set up and use a dedicated Python virtual environment for isolated dependency management:

### Linux / macOS

```bash
# Create a virtual environment using Python 3.11+
python3.11 -m venv .venv

# Activate the virtual environment
source .venv/bin/activate

# Upgrade pip and install maturin
pip install --upgrade pip maturin
```

### Windows

```bat
# Create a virtual environment using Python 3.11+
py -3.11 -m venv .venv

# Activate the virtual environment
.venv\Scripts\activate

# Upgrade pip and install maturin
pip install --upgrade pip maturin
```

## Python Installation

Once your virtual environment is active:

```bash
# Development editable install
maturin develop

# Optimized release install
maturin develop --release
```

## Python Usage Examples

### Synchronous Usage

```python
from rust_shdlc_driver import ShdlcSerialPort, ShdlcConnection, ShdlcDevice

with ShdlcSerialPort(port="/dev/ttyUSB0", baudrate=115200) as port:
    conn = ShdlcConnection(port)
    device = ShdlcDevice(conn, slave_address=0)
    
    product_type = device.get_product_type()
    product_name = device.get_product_name()
    serial_number = device.get_serial_number()
    version = device.get_version()
    
    print(f"Product: {product_name} ({product_type})")
    print(f"Serial: {serial_number}")
    print(f"Version: {version}")
```

### Asynchronous Usage (Asyncio)

```python
import asyncio
from rust_shdlc_driver import ShdlcSerialPort, AsyncShdlcConnection, AsyncShdlcDevice

async def main():
    port = ShdlcSerialPort(port="/dev/ttyUSB0", baudrate=115200)
    conn = AsyncShdlcConnection(port)
    device = AsyncShdlcDevice(conn, slave_address=0)
    
    name = await device.get_product_name()
    print(f"Product name: {name}")

asyncio.run(main())
```

### Virtual Mock Port for Testing

```python
from rust_shdlc_driver import ShdlcMockPort, ShdlcConnection, ShdlcDevice

port = ShdlcMockPort(bitrate=115200)
conn = ShdlcConnection(port)
device = ShdlcDevice(conn, slave_address=1)

# Push response frame to virtual mock queue
port.push_rx_data(b"\x7e\x01\xd0\x00\x06SHT31\0\x7d\x5d\x7e")
assert device.get_product_name() == "SHT31"
```

## Running Tests

```bash
# Rust unit and mock integration tests
cargo test

# Python pytest suite
pytest python_tests/ -v
```

## Building Documentation

```bash
pip install sphinx sphinx_rtd_theme
sphinx-build -b html docs docs/_build/html
```

## License

See [LICENSE](LICENSE).
