# -*- coding: utf-8 -*-
import pytest
from rust_shdlc_driver import (
    ShdlcMockPort,
    AsyncShdlcConnection,
    AsyncShdlcDevice,
)


def make_resp(addr, cmd, state, data):
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


@pytest.mark.asyncio
async def test_async_device_operations():
    port = ShdlcMockPort(bitrate=115200)
    conn = AsyncShdlcConnection(port)
    dev = AsyncShdlcDevice(conn, slave_address=1)

    port.push_rx_data(make_resp(1, 0xD0, 0x00, b"SEN54\0"))
    name = await dev.get_product_name()
    assert name == "SEN54"

    port.push_rx_data(make_resp(1, 0xD0, 0x00, b"00050000\0"))
    pt = await dev.get_product_type()
    assert pt == "00050000"

    port.push_rx_data(make_resp(1, 0xD0, 0x00, b"00050000\0"))
    pt_int = await dev.get_product_type(as_int=True)
    assert pt_int == 0x00050000

    port.push_rx_data(make_resp(1, 0x90, 0x00, b"\x01"))
    addr = await dev.get_slave_address()
    assert addr == 1

    port.push_rx_data(make_resp(1, 0x90, 0x00, b""))
    await dev.set_slave_address(3, update_driver=True)
    assert dev.slave_address == 3

    port.push_rx_data(make_resp(3, 0x91, 0x00, b"\x00\x01\xc2\x00"))
    baud = await dev.get_baudrate()
    assert baud == 115200

    port.push_rx_data(make_resp(3, 0x91, 0x00, b""))
    await dev.set_baudrate(9600, update_driver=True)

    port.push_rx_data(make_resp(3, 0x95, 0x00, b"\x00\x14"))
    delay = await dev.get_reply_delay()
    assert delay == 20

    port.push_rx_data(make_resp(3, 0x93, 0x00, b"\x00\x00\x00\x64"))
    uptime = await dev.get_system_up_time()
    assert uptime == 100
