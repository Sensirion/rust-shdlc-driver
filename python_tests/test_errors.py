# -*- coding: utf-8 -*-
import pytest
from rust_shdlc_driver import (
    ShdlcError,
    ShdlcTimeoutError,
    ShdlcResponseError,
    ShdlcDeviceError,
    ShdlcCommandDataSizeError,
    ShdlcUnknownCommandError,
    ShdlcAccessRightError,
    ShdlcCommandParameterError,
    ShdlcChecksumError,
    ShdlcFirmwareUpdateError,
    ShdlcFirmwareImageSignatureError,
    ShdlcFirmwareImageIncompatibilityError,
)


def test_exception_hierarchy():
    assert issubclass(ShdlcTimeoutError, ShdlcError)
    assert issubclass(ShdlcResponseError, ShdlcError)
    assert issubclass(ShdlcDeviceError, ShdlcError)
    assert issubclass(ShdlcCommandDataSizeError, ShdlcDeviceError)
    assert issubclass(ShdlcUnknownCommandError, ShdlcDeviceError)
    assert issubclass(ShdlcAccessRightError, ShdlcDeviceError)
    assert issubclass(ShdlcCommandParameterError, ShdlcDeviceError)
    assert issubclass(ShdlcChecksumError, ShdlcDeviceError)
    assert issubclass(ShdlcFirmwareUpdateError, ShdlcDeviceError)
    assert issubclass(ShdlcFirmwareImageSignatureError, ShdlcError)
    assert issubclass(ShdlcFirmwareImageIncompatibilityError, ShdlcError)


def test_response_error():
    err = ShdlcResponseError("Bad payload")
    assert "Bad payload" in str(err)
