rust-shdlc-driver Documentation
================================

Welcome to the documentation of **rust-shdlc-driver**, a high-performance async Rust SHDLC driver and Python package (Python >= 3.11) with PyO3 bindings.

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   architecture
   build
   quickstart
   api
   firmware

Overview
--------

`rust-shdlc-driver` implements the Sensirion SHDLC communication protocol with a clean separation of the transport layer (serial / TCP / mock) and the protocol layer (framing, byte-stuffing, checksums, commands).

Key features:
- **Decoupled Transport**: Pure trait abstraction ``ShdlcTransport`` for Serial UART, TCP sockets, and in-memory mock transports.
- **Async & Sync Support**: Async-first architecture powered by Tokio, alongside blocking sync wrappers for standard scripts.
- **Python >= 3.11 Bindings**: Native PyO3 extension module supporting both synchronous workflows and Python ``asyncio``.
- **Test-Driven Design**: Full test suite with unit tests and mock transports.
- **Firmware Updates**: Full support for parsing Intel-Hex firmware images, signature checking, and flashing over SHDLC bootloader.
