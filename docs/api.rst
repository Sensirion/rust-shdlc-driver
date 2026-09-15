API Reference
=============

Exceptions
----------

- ``ShdlcError``: Base exception class.
- ``ShdlcTimeoutError``: Raised when a response times out.
- ``ShdlcResponseError``: Raised when an invalid response frame is received. Has ``received_data`` property.
- ``ShdlcDeviceError``: Raised when device returns an error code. Has ``error_code`` and ``error_message`` properties.
- ``ShdlcCommandDataSizeError``: Error code 1 (data size error).
- ``ShdlcUnknownCommandError``: Error code 2 (unknown command).
- ``ShdlcAccessRightError``: Error code 3 (access rights error).
- ``ShdlcCommandParameterError``: Error code 4 (parameter out of range).
- ``ShdlcChecksumError``: Error code 5 (checksum error).
- ``ShdlcFirmwareUpdateError``: Error code 6 (update error).
- ``ShdlcFirmwareImageSignatureError``: Invalid signature in hex image.
- ``ShdlcFirmwareImageIncompatibilityError``: Firmware image incompatible with device product type.

Version Types
-------------

- ``FirmwareVersion(major: int, minor: int, debug: bool = False)``
- ``HardwareVersion(major: int, minor: int)``
- ``ProtocolVersion(major: int, minor: int)``
- ``Version(firmware: FirmwareVersion, hardware: HardwareVersion, protocol: ProtocolVersion)``

Ports
-----

- ``ShdlcSerialPort(port: str, baudrate: int, additional_response_time: float = 0.1, do_open: bool = True)``
- ``ShdlcTcpPort(ip: str, port: int, socket_timeout: float = 5.0, do_open: bool = True)``
- ``ShdlcMockPort(bitrate: int = 115200)``

Connection
----------

- ``ShdlcConnection(port)``: Synchronous connection wrapper.
- ``AsyncShdlcConnection(port)``: Asynchronous connection wrapper.

Device
------

- ``ShdlcDevice(connection, slave_address: int)``: Synchronous device driver.
- ``AsyncShdlcDevice(connection, slave_address: int)``: Asynchronous device driver.
