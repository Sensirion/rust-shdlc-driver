Quickstart & Usage Examples
============================

Prerequisites & Virtual Environment Setup
-----------------------------------------

We recommend setting up and activating a dedicated virtual environment:

.. code-block:: bash

   # Create and activate virtual environment (Linux/macOS)
   python3.11 -m venv .venv
   source .venv/bin/activate

   # On Windows:
   # py -3.11 -m venv .venv
   # .venv\Scripts\activate

   # Install the driver
   maturin develop --release

Synchronous Usage (Serial Port)
-------------------------------

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

Asynchronous Usage (asyncio)
----------------------------

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
--------------------------------

.. code-block:: python

   from rust_shdlc_driver import ShdlcMockPort, ShdlcConnection, ShdlcDevice

   # Create a virtual mock port
   mock_port = ShdlcMockPort(bitrate=115200)
   conn = ShdlcConnection(mock_port)
   device = ShdlcDevice(conn, slave_address=1)

   # Push a mock response frame
   mock_port.push_rx_data(b"\x7e\x01\xd0\x00\x06SHT31\0\x7d\x5d\x7e")

   name = device.get_product_name()
   assert name == "SHT31"
