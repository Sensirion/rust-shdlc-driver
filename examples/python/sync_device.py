#!/usr/bin/env python3
"""Example of synchronous SHDLC device communication over serial."""

from rust_shdlc_driver import ShdlcConnection, ShdlcDevice, ShdlcSerialPort


def main():
    # Connect to device on /dev/ttyUSB0 with baudrate 115200
    with ShdlcSerialPort(port="/dev/ttyUSB0", baudrate=115200) as port:
        conn = ShdlcConnection(port)
        device = ShdlcDevice(conn, slave_address=0)

        product_name = device.get_product_name()
        product_type = device.get_product_type()
        serial_number = device.get_serial_number()
        version = device.get_version()

        print(f"Product Name: {product_name}")
        print(f"Product Type: {product_type}")
        print(f"Serial Number: {serial_number}")
        print(f"Version: {version}")


if __name__ == "__main__":
    main()
