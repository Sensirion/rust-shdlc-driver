# -*- coding: utf-8 -*-
import threading
import time

from rust_shdlc_driver import (
    ShdlcConnection,
    ShdlcDevice,
    ShdlcMockPort,
    ShdlcUnknownCommandError,
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


def test_blocking_requests_detach_gil():
    port = ShdlcMockPort(bitrate=115200)
    conn = ShdlcConnection(port)
    dev = ShdlcDevice(conn, slave_address=1)

    thread_ran = False

    def delayed_response():
        nonlocal thread_ran
        time.sleep(0.05)
        thread_ran = True
        port.push_rx_data(make_resp(1, 0xD0, 0x00, b"SGP40\0"))

    t = threading.Thread(target=delayed_response)
    t.start()

    res = dev.get_product_name()
    t.join()

    assert res == "SGP40"
    assert thread_ran is True


def test_sync_device_operations():
    port = ShdlcMockPort(bitrate=115200)
    conn = ShdlcConnection(port)
    dev = ShdlcDevice(conn, slave_address=1)

    # get_product_type
    port.push_rx_data(make_resp(1, 0xD0, 0x00, b"00080000\0"))
    assert dev.get_product_type() == "00080000"
    port.push_rx_data(make_resp(1, 0xD0, 0x00, b"00080000\0"))
    assert dev.get_product_type(as_int=True) == 0x00080000

    # get_product_name
    port.push_rx_data(make_resp(1, 0xD0, 0x00, b"SGP40\0"))
    assert dev.get_product_name() == "SGP40"

    # get_serial_number
    port.push_rx_data(make_resp(1, 0xD0, 0x00, b"SN123456\0"))
    assert dev.get_serial_number() == "SN123456"

    # get_version
    port.push_rx_data(make_resp(1, 0xD1, 0x00, b"\x01\x02\x00\x03\x00\x01\x00"))
    v = dev.get_version()
    assert v.firmware.major == 1
    assert v.firmware.minor == 2
    assert v.hardware.major == 3
    assert v.protocol.major == 1

    # get_error_state
    port.push_rx_data(make_resp(1, 0xD2, 0x00, b"\x00\x00\x00\x01\x02"))
    state, err = dev.get_error_state(clear=True, as_exception=True)
    assert state == 1
    assert isinstance(err, ShdlcUnknownCommandError)

    # get_slave_address
    port.push_rx_data(make_resp(1, 0x90, 0x00, b"\x01"))
    assert dev.get_slave_address() == 1

    # set_slave_address
    port.push_rx_data(make_resp(1, 0x90, 0x00, b""))
    dev.set_slave_address(2, update_driver=True)
    assert dev.slave_address == 2

    # get_baudrate
    port.push_rx_data(make_resp(2, 0x91, 0x00, b"\x00\x01\xc2\x00"))
    assert dev.get_baudrate() == 115200

    # set_baudrate
    port.push_rx_data(make_resp(2, 0x91, 0x00, b""))
    dev.set_baudrate(9600, update_driver=True)
    assert port.bitrate == 9600

    # get_reply_delay
    port.push_rx_data(make_resp(2, 0x95, 0x00, b"\x00\x32"))
    assert dev.get_reply_delay() == 50

    # set_reply_delay
    port.push_rx_data(make_resp(2, 0x95, 0x00, b""))
    dev.set_reply_delay(50)

    # get_system_up_time
    port.push_rx_data(make_resp(2, 0x93, 0x00, b"\x00\x00\x01\x00"))
    assert dev.get_system_up_time() == 256
