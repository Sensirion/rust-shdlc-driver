#!/usr/bin/env python3
"""Example of asynchronous SHDLC device communication using asyncio."""

import asyncio
from rust_shdlc_driver import AsyncShdlcConnection, AsyncShdlcDevice, ShdlcSerialPort


async def main():
    # Connect to device on /dev/ttyUSB0 with baudrate 115200
    port = ShdlcSerialPort(port="/dev/ttyUSB0", baudrate=115200)
    conn = AsyncShdlcConnection(port)
    device = AsyncShdlcDevice(conn, slave_address=0)

    product_name = await device.get_product_name()
    product_type = await device.get_product_type()
    serial_number = await device.get_serial_number()
    version = await device.get_version()

    print(f"Product Name: {product_name}")
    print(f"Product Type: {product_type}")
    print(f"Serial Number: {serial_number}")
    print(f"Version: {version}")


if __name__ == "__main__":
    asyncio.run(main())
