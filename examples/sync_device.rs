use rust_shdlc_driver::connection::ShdlcConnection;
use rust_shdlc_driver::device::ShdlcDevice;
use rust_shdlc_driver::transport::AsyncSerialPort;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open serial port at 115200 baud
    let mut port = AsyncSerialPort::new("/dev/ttyUSB0", 115200);
    port.open()?;

    let conn = ShdlcConnection::new(Box::new(port))?;
    let mut device = ShdlcDevice::new(conn, 0);

    // Read device information
    let product_name = device.get_product_name()?;
    let product_type = device.get_product_type()?;
    let serial_number = device.get_serial_number()?;
    let version = device.get_version()?;

    println!("Product Name: {}", product_name);
    println!("Product Type: {}", product_type);
    println!("Serial Number: {}", serial_number);
    println!(
        "Firmware Version: {}.{}",
        version.firmware.major, version.firmware.minor
    );

    Ok(())
}
