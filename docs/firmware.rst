Firmware Update
===============

The driver includes support for updating device firmware using the SHDLC bootloader protocol.

Loading Firmware Images
-----------------------

.. code-block:: python

   from rust_shdlc_driver import ShdlcFirmwareImage

   # Load an Intel-Hex firmware image
   image = ShdlcFirmwareImage(
       hexfile="path/to/firmware.hex",
       bl_start_addr=0x08000000,
       app_start_addr=0x08004000
   )

   print(f"Product Type: 0x{image.product_type:08X}")
   print(f"Bootloader Version: {image.bootloader_version}")
   print(f"Application Version: {image.application_version}")
   print(f"Image Size: {image.size} bytes")
   print(f"Checksum: 0x{image.checksum:02X}")

Executing Firmware Updates
--------------------------

.. code-block:: python

   from rust_shdlc_driver import (
       ShdlcSerialPort,
       ShdlcConnection,
       ShdlcDevice,
       ShdlcFirmwareImage,
       ShdlcFirmwareUpdate,
   )

   port = ShdlcSerialPort(port="/dev/ttyUSB0", baudrate=115200)
   conn = ShdlcConnection(port)
   device = ShdlcDevice(conn, slave_address=0)

   image = ShdlcFirmwareImage("firmware.hex", 0x08000000, 0x08004000)

   updater = ShdlcFirmwareUpdate(
       device=device,
       image=image,
       status_callback=lambda status: print(f"[STATUS] {status}"),
       progress_callback=lambda progress: print(f"[PROGRESS] {progress:.1f}%")
   )

   # Execute firmware update
   updater.execute(emergency=False)
   print("Update completed successfully!")
