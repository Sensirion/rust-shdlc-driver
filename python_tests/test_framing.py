# -*- coding: utf-8 -*-
import pytest
from rust_shdlc_driver import (
    ShdlcSerialMosiFrameBuilder,
    ShdlcSerialMisoFrameBuilder,
    ShdlcResponseError,
)


@pytest.mark.parametrize(
    "address,command,data,expected",
    [
        (0x00, 0x00, b"", b"\x7e\x00\x00\x00\xff\x7e"),
        (
            0x00,
            0x00,
            b"\x00" * 255,
            b"\x7e\x00\x00\xff" + b"\x00" * 255 + b"\x00\x7e",
        ),
        (
            0xFF,
            0xFF,
            b"\xff" * 255,
            b"\x7e\xff\xff\xff" + b"\xff" * 255 + b"\x01\x7e",
        ),
        (
            0x7E,
            0x7D,
            b"\x11\x12\x13\x14",
            b"\x7e\x7d\x5e\x7d\x5d\x04\x7d\x31\x12\x7d\x33\x14\xb6\x7e",
        ),
    ],
)
def test_mosi_frame_builder(address, command, data, expected):
    builder = ShdlcSerialMosiFrameBuilder(address, command, data)
    assert builder.to_bytes() == expected


def test_miso_frame_builder_basic():
    builder = ShdlcSerialMisoFrameBuilder()
    assert builder.start_received is False
    assert len(builder.data) == 0

    assert builder.add_data(b"\x7e\x00\x00\x00\x00\xff\x7e") is True
    assert builder.start_received is True

    addr, cmd, state, data = builder.interpret_data()
    assert addr == 0x00
    assert cmd == 0x00
    assert state == 0x00
    assert data == b""


def test_miso_frame_builder_stuffing():
    builder = ShdlcSerialMisoFrameBuilder()
    raw = b"\x7e\x7d\x5e\x7d\x5d\x7d\x31\x03\x12\x7d\x33\x14\xb7\x7e"
    assert builder.add_data(raw) is True
    addr, cmd, state, data = builder.interpret_data()
    assert addr == 0x7E
    assert cmd == 0x7D
    assert state == 0x11
    assert data == b"\x12\x13\x14"


def test_miso_frame_builder_invalid_checksum():
    builder = ShdlcSerialMisoFrameBuilder()
    builder.add_data(b"\x7e\x00\x00\x00\x00\xfe\x7e")
    with pytest.raises(ShdlcResponseError):
        builder.interpret_data()
