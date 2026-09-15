Architecture & Design
=====================

The architecture of `rust-shdlc-driver` strictly decouples the **Protocol Layer** from the **Transport Layer**.

Layer Separation
----------------

::

   +-------------------------------------------------------+
   |             Application / Device Layer                |
   |      (ShdlcDevice, AsyncShdlcDevice, FirmwareUpdate)   |
   +-------------------------------------------------------+
                              |
   +-------------------------------------------------------+
   |                 SHDLC Bus Connection                  |
   |     (ShdlcConnection, AsyncShdlcConnection)           |
   +-------------------------------------------------------+
                              |
   +-------------------------------------------------------+
   |                    Protocol Layer                     |
   |  (MOSI/MISO Frame Builders, Byte Stuffing, Checksum)  |
   +-------------------------------------------------------+
                              |
   +-------------------------------------------------------+
   |            Transport Abstraction (Trait)              |
   |                   ShdlcTransport                      |
   +-------------------------------------------------------+
          /                   |                   \
   +----------------+ +----------------+ +----------------+
   |   AsyncSerial  | |    AsyncTcp    | |  MockTransport |
   |  (tokio-serial)| |  (tokio::net)  | |  (Unit Tests)  |
   +----------------+ +----------------+ +----------------+

Protocol Layer
--------------

The SHDLC protocol specifies:

- **Frame Delimiters**: Start and Stop byte ``0x7E``
- **Byte Stuffing**: Escaping reserved bytes ``0x7E``, ``0x7D``, ``0x11``, and ``0x13`` with escape byte ``0x7D`` and XOR ``0x20``.
- **Checksum**: Inverted sum of all unescaped bytes modulo 256: ``(~sum) & 0xFF``.
- **MOSI Frame Structure**: ``[START, Stuffed(Address, Command, Length, Data..., Checksum), STOP]``.
- **MISO Frame Structure**: ``[START, Stuffed(Address, Command, State, Length, Data..., Checksum), STOP]``.

Transport Layer
---------------

The `ShdlcTransport` async trait abstracts I/O operations:

- ``write_all(&mut self, data: &[u8])``
- ``read(&mut self, buf: &mut [u8]) -> usize``
- ``flush(&mut self)``
- ``set_bitrate(&mut self, bitrate: u32)``
- ``bitrate(&self) -> u32``
- ``is_open(&self) -> bool``
- ``close(&mut self)``
- ``description(&self) -> String``

This allows replacing hardware serial ports with TCP streams or memory mock streams seamlessly.
