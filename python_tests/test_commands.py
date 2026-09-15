# -*- coding: utf-8 -*-
from rust_shdlc_driver import (
    ShdlcCommand,
    ShdlcCmdGetProductType,
    ShdlcCmdGetProductName,
    ShdlcCmdGetArticleCode,
    ShdlcCmdGetSerialNumber,
    ShdlcCmdGetProductSubType,
    ShdlcCmdGetVersion,
    ShdlcCmdGetErrorState,
    ShdlcCmdDeviceReset,
    ShdlcCmdGetSlaveAddress,
    ShdlcCmdSetSlaveAddress,
    ShdlcCmdGetBaudrate,
    ShdlcCmdSetBaudrate,
    ShdlcCmdGetReplyDelay,
    ShdlcCmdSetReplyDelay,
    ShdlcCmdGetSystemUpTime,
    ShdlcCmdFactoryReset,
    ShdlcCmdEnterBootloader,
    ShdlcCmdFirmwareUpdateStart,
    ShdlcCmdFirmwareUpdateData,
    ShdlcCmdFirmwareUpdateStop,
)


def test_command_defaults():
    cmd = ShdlcCmdGetProductType()
    assert cmd.id == 0xD0
    assert bytes(cmd.data) == b"\x00"
    assert cmd.interpret_response(b"00000001\0") == "00000001"

    cmd_sub = ShdlcCmdGetProductSubType()
    assert cmd_sub.interpret_response(b"\x03") == 3

    cmd_get_addr = ShdlcCmdGetSlaveAddress()
    assert cmd_get_addr.id == 0x90
    assert bytes(cmd_get_addr.data) == b""
    assert cmd_get_addr.interpret_response(b"\x2a") == 42

    cmd_set_addr = ShdlcCmdSetSlaveAddress(42)
    assert cmd_set_addr.id == 0x90
    assert bytes(cmd_set_addr.data) == b"\x2a"
    assert cmd_set_addr.interpret_response(b"") is None

    cmd_get_baud = ShdlcCmdGetBaudrate()
    assert cmd_get_baud.id == 0x91
    assert bytes(cmd_get_baud.data) == b""
    assert cmd_get_baud.interpret_response(b"\x00\x01\xc2\x00") == 115200

    cmd_baud = ShdlcCmdSetBaudrate(115200)
    assert cmd_baud.id == 0x91
    assert bytes(cmd_baud.data) == b"\x00\x01\xc2\x00"
    assert cmd_baud.interpret_response(b"") is None

    cmd_get_delay = ShdlcCmdGetReplyDelay()
    assert cmd_get_delay.id == 0x95
    assert bytes(cmd_get_delay.data) == b""
    assert cmd_get_delay.interpret_response(b"\x00\x32") == 50

    cmd_set_delay = ShdlcCmdSetReplyDelay(50)
    assert cmd_set_delay.id == 0x95
    assert bytes(cmd_set_delay.data) == b"\x00\x32"

    cmd_uptime = ShdlcCmdGetSystemUpTime()
    assert cmd_uptime.id == 0x93
    assert bytes(cmd_uptime.data) == b""
    assert cmd_uptime.interpret_response(b"\x00\x00\x04\xd2") == 1234

    cmd_ver = ShdlcCmdGetVersion()
    assert cmd_ver.id == 0xD1
    v = cmd_ver.interpret_response(b"\x01\x02\x00\x03\x04\x01\x00")
    assert v.firmware.major == 1
    assert v.firmware.minor == 2
    assert not v.firmware.debug
    assert v.hardware.major == 3
    assert v.hardware.minor == 4
    assert v.protocol.major == 1
    assert v.protocol.minor == 0

    cmd_err = ShdlcCmdGetErrorState(clear=True)
    assert cmd_err.id == 0xD2
    assert bytes(cmd_err.data) == b"\x01"
    assert cmd_err.interpret_response(b"\x00\x00\x00\x10\x03") == (16, 3)

    cmd_custom = ShdlcCommand(0x10, b"\x01\x02", 0.1)
    assert cmd_custom.id == 0x10
    assert bytes(cmd_custom.data) == b"\x01\x02"
