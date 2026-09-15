# -*- coding: utf-8 -*-
from rust_shdlc_driver import (
    FirmwareVersion,
    HardwareVersion,
    ProtocolVersion,
    Version,
)


def test_firmware_version():
    fv = FirmwareVersion(1, 2, False)
    assert fv.major == 1
    assert fv.minor == 2
    assert fv.debug is False
    assert str(fv) == "1.2"
    assert repr(fv) == "FirmwareVersion(major=1, minor=2, debug=false)"

    fv_debug = FirmwareVersion(2, 5, True)
    assert fv_debug.major == 2
    assert fv_debug.minor == 5
    assert fv_debug.debug is True
    assert str(fv_debug) == "2.5-debug"


def test_hardware_version():
    hv = HardwareVersion(3, 0)
    assert hv.major == 3
    assert hv.minor == 0
    assert str(hv) == "3.0"


def test_protocol_version():
    pv = ProtocolVersion(1, 0)
    assert pv.major == 1
    assert pv.minor == 0
    assert str(pv) == "1.0"


def test_version():
    fv = FirmwareVersion(1, 2, False)
    hv = HardwareVersion(3, 0)
    pv = ProtocolVersion(1, 0)
    v = Version(fv, hv, pv)

    assert v.firmware == fv
    assert v.hardware == hv
    assert v.protocol == pv
    assert str(v) == "Firmware 1.2, Hardware 3.0, Protocol 1.0"
