Quickstart & Usage Examples
============================

Rust Quickstart
---------------

Add ``rust-shdlc-driver`` to your ``Cargo.toml``:

.. code-block:: toml

   [dependencies]
   rust-shdlc-driver = "0.1.0"

Synchronous Usage (Rust)
^^^^^^^^^^^^^^^^^^^^^^^^

.. code-block:: rust

   use rust_shdlc_driver::connection::ShdlcConnection;
   use rust_shdlc_driver::device::ShdlcDevice;
   use rust_shdlc_driver::transport::AsyncSerialPort;

   fn main() -> Result<(), Box<dyn std::error::Error>> {
       let mut port = AsyncSerialPort::new("/dev/ttyUSB0", 115200);
       port.open()?;

       let conn = ShdlcConnection::new(Box::new(port))?;
       let mut device = ShdlcDevice::new(conn, 0);

       let product_name = device.get_product_name()?;
       let serial_number = device.get_serial_number()?;

       println!("Connected to {}, S/N: {}", product_name, serial_number);
       Ok(())
   }

Asynchronous Usage (Rust / Tokio)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. code-block:: rust

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

       println!("Connected to {}, S/N: {}", product_name, serial_number);
       Ok(())
   }

Python Quickstart
-----------------

Prerequisites & Virtual Environment Setup
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

We recommend setting up and activating a dedicated virtual environment:

.. code-block:: bash

   # Create and activate virtual environment (Linux/macOS)
   python3.11 -m venv .venv
   source .venv/bin/activate

   # On Windows:
   # py -3.11 -m venv .venv
   # .venv\Scripts\activate

   # Install the driver via pip
   pip install rust-shdlc-driver

Synchronous Usage (Python Serial Port)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. code-block:: python

   from rust_shdlc_driver import ShdlcSerialPort, ShdlcConnection, ShdlcDevice

   # Open serial port with baudrate 115200
   with ShdlcSerialPort(port="/dev/ttyUSB0", baudrate=115200) as port:
       conn = ShdlcConnection(port)
       device = ShdlcDevice(conn, slave_address=0)

       # Read identification
       prod_type = device.get_product_type()
       prod_name = device.get_product_name()
       serial_nr = device.get_serial_number()
       version = device.get_version()

       print(f"Connected to {prod_name} (Type: {prod_type}, S/N: {serial_nr})")
       print(f"Versions: {version}")

Asynchronous Usage (Python asyncio)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. code-block:: python

   import asyncio
   from rust_shdlc_driver import ShdlcSerialPort, AsyncShdlcConnection, AsyncShdlcDevice

   async def main():
       port = ShdlcSerialPort(port="/dev/ttyUSB0", baudrate=115200)
       conn = AsyncShdlcConnection(port)
       device = AsyncShdlcDevice(conn, slave_address=0)

       # Query device asynchronously
       name = await device.get_product_name()
       print(f"Device name: {name}")

   asyncio.run(main())

Testing with In-Memory Mock Port
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. code-block:: python

   from rust_shdlc_driver import ShdlcMockPort, ShdlcConnection, ShdlcDevice

   # Create a virtual mock port
   mock_port = ShdlcMockPort(bitrate=115200)
   conn = ShdlcConnection(mock_port)
   device = ShdlcDevice(conn, slave_address=1)

   # Push a mock response frame (GetProductName -> SHT31)
   mock_port.push_rx_data(b"\x7e\x01\xd0\x00\x06SHT31\0\xd5\x7e")

   name = device.get_product_name()
   assert name == "SHT31"
