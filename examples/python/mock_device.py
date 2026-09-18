#!/usr/bin/env python3
"""Example of SHDLC communication using virtual in-memory mock port."""

from rust_shdlc_driver import ShdlcConnection, ShdlcDevice, ShdlcMockPort


def make_resp(addr: int, cmd: int, state: int, data: bytes) -> bytes:
    content = bytearray([addr, cmd, state, len(data)]) + bytearray(data)
    checksum = (~sum(content)) & 0xFF
    content.append(checksum)
    stuffed = bytearray()
    for b in content:
        if b in [0x7E, 0x7D, 0x11, 0x13]:
            stuffed.append(0x7D)
            stuffed.append(b ^ 0x20)
        else:
            stuffed.append(b)
    return bytes(bytearray([0x7E]) + stuffed + bytearray([0x7E]))


def main():
    # Create mock port and push a simulated SHDLC response for GetProductName
    port = ShdlcMockPort(bitrate=115200)
    port.push_rx_data(make_resp(1, 0xD0, 0x00, b"SHT31\0"))

    conn = ShdlcConnection(port)
    device = ShdlcDevice(conn, slave_address=1)

    name = device.get_product_name()
    print(f"Mock device product name: {name}")
    assert name == "SHT31"


if __name__ == "__main__":
    main()
