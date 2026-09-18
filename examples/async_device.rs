use rust_shdlc_driver::connection::AsyncShdlcConnection;
use rust_shdlc_driver::device::AsyncShdlcDevice;
use rust_shdlc_driver::transport::AsyncSerialPort;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open serial port at 115200 baud
    let mut port = AsyncSerialPort::new("/dev/ttyUSB0", 115200);
    port.open()?;

    let conn = AsyncShdlcConnection::new(Box::new(port));
    let mut device = AsyncShdlcDevice::new(conn, 0);

    // Read device information asynchronously
    let product_name = device.get_product_name().await?;
    let product_type = device.get_product_type().await?;
    let serial_number = device.get_serial_number().await?;
    let version = device.get_version().await?;

    println!("Product Name: {}", product_name);
    println!("Product Type: {}", product_type);
    println!("Serial Number: {}", serial_number);
    println!(
        "Firmware Version: {}.{}",
        version.firmware.major, version.firmware.minor
    );

    Ok(())
}
