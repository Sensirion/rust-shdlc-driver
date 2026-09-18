use rust_shdlc_driver::connection::ShdlcConnection;
use rust_shdlc_driver::device::ShdlcDevice;
use rust_shdlc_driver::protocol::{calculate_checksum, stuff_data_bytes, START_STOP_BYTE};
use rust_shdlc_driver::transport::MockTransport;

fn make_response_frame(slave_addr: u8, cmd_id: u8, state: u8, data: &[u8]) -> Vec<u8> {
    let mut content = vec![slave_addr, cmd_id, state, data.len() as u8];
    content.extend_from_slice(data);
    let checksum = calculate_checksum(&content);
    content.push(checksum);
    let stuffed = stuff_data_bytes(&content);
    let mut raw = vec![START_STOP_BYTE];
    raw.extend_from_slice(&stuffed);
    raw.push(START_STOP_BYTE);
    raw
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mock = MockTransport::new();
    // Simulate a device at address 0x00 responding to GetProductName (0xD0) with "SHT31\0"
    mock.push_rx_data(&make_response_frame(0x00, 0xD0, 0x00, b"SHT31\0"));

    let conn = ShdlcConnection::new(Box::new(mock))?;
    let mut device = ShdlcDevice::new(conn, 0);

    let product_name = device.get_product_name()?;
    println!("Device responded with product name: {}", product_name);
    assert_eq!(product_name, "SHT31");

    Ok(())
}
