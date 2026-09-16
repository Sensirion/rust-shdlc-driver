# -*- coding: utf-8 -*-
import os
import pytest
from rust_shdlc_driver import (
    AsyncShdlcConnection,
    AsyncShdlcDevice,
    AsyncShdlcFirmwareUpdate,
    ShdlcConnection,
    ShdlcDevice,
    ShdlcFirmwareImage,
    ShdlcFirmwareImageSignatureError,
    ShdlcFirmwareUpdate,
    ShdlcMockPort,
)

DATA_DIR = os.path.join(os.path.dirname(__file__), "..", "tests", "data")
EKS2_HEX = os.path.join(DATA_DIR, "Eks2_combined_V5.2.hex")
STM32_HEX = os.path.join(DATA_DIR, "Stm32g0Firmware.hex")


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


def test_firmware_image_properties():
    img = ShdlcFirmwareImage(EKS2_HEX, 0x08000000, 0x08004000)
    assert img.product_type == 0x00060000
    assert img.bootloader_version.major == 0
    assert img.bootloader_version.minor == 4
    assert img.application_version.major == 5
    assert img.application_version.minor == 2
    assert img.size == 75096
    assert img.checksum == 0x88
    assert img.available_bytes == 75096

    chunk = img.read(10)
    assert len(chunk) == 10
    assert img.available_bytes == 75096 - 10


def test_firmware_invalid_sig():
    with pytest.raises(ShdlcFirmwareImageSignatureError):
        ShdlcFirmwareImage(EKS2_HEX, 0x08000000, 0x08004001)


def test_firmware_update_sync():
    img = ShdlcFirmwareImage(
        STM32_HEX,
        0x08000000,
        0x08001000,
        signature=b"\x4b\x4f\x47\x4a\xa4\x74\xf4\xb4",
        bl_version_offset=0x200,
    )

    port = ShdlcMockPort(bitrate=115200)
    conn = ShdlcConnection(port)
    dev = ShdlcDevice(conn, slave_address=0)

    # 1. Product type response: "00140000\0"
    port.push_rx_data(make_resp(0, 0xD0, 0, b"00140000\0"))
    # 2. EnterBootloader
    port.push_rx_data(make_resp(0, 0xF3, 0, b""))
    # 3. Start
    port.push_rx_data(make_resp(0, 0xF3, 0, b""))
    # 4. Data blocks: 1600 / 254 = 7 blocks
    for _ in range(7):
        port.push_rx_data(make_resp(0, 0xF3, 0, b""))
    # 5. Stop
    port.push_rx_data(make_resp(0, 0xF3, 0, b""))

    statuses = []
    progresses = []
    updater = ShdlcFirmwareUpdate(
        dev,
        img,
        status_callback=lambda s: statuses.append(s),
        progress_callback=lambda p: progresses.append(p),
    )
    updater.execute(emergency=False)

    assert len(statuses) > 0
    assert len(progresses) > 0
    assert progresses[-1] == 100.0


@pytest.mark.asyncio
async def test_firmware_update_async():
    img = ShdlcFirmwareImage(
        STM32_HEX,
        0x08000000,
        0x08001000,
        signature=b"\x4b\x4f\x47\x4a\xa4\x74\xf4\xb4",
        bl_version_offset=0x200,
    )

    port = ShdlcMockPort(bitrate=115200)
    conn = AsyncShdlcConnection(port)
    dev = AsyncShdlcDevice(conn, slave_address=0)

    # 1. Product type response: "00140000\0"
    port.push_rx_data(make_resp(0, 0xD0, 0, b"00140000\0"))
    # 2. EnterBootloader
    port.push_rx_data(make_resp(0, 0xF3, 0, b""))
    # 3. Start
    port.push_rx_data(make_resp(0, 0xF3, 0, b""))
    # 4. Data blocks
    for _ in range(7):
        port.push_rx_data(make_resp(0, 0xF3, 0, b""))
    # 5. Stop
    port.push_rx_data(make_resp(0, 0xF3, 0, b""))

    updater = AsyncShdlcFirmwareUpdate(dev, img)
    await updater.execute(emergency=False)
