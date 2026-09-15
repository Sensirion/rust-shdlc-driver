use rust_shdlc_driver::connection::AsyncShdlcConnection;
use rust_shdlc_driver::device::AsyncShdlcDevice;
use rust_shdlc_driver::protocol::errors::ShdlcError;
use rust_shdlc_driver::protocol::*;
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

#[tokio::test]
async fn test_async_device_operations() {
    let mock = MockTransport::new();
    let conn = AsyncShdlcConnection::new(Box::new(mock.clone()));
    let mut device = AsyncShdlcDevice::new(conn, 0x01);

    // Test get_product_type
    mock.push_rx_data(&make_response_frame(0x01, 0xD0, 0x00, b"00030000\0"));
    let pt = device.get_product_type().await.unwrap();
    assert_eq!(pt, "00030000");

    // Test get_product_name
    mock.push_rx_data(&make_response_frame(0x01, 0xD0, 0x00, b"SHT31\0"));
    let name = device.get_product_name().await.unwrap();
    assert_eq!(name, "SHT31");

    // Test get_serial_number
    mock.push_rx_data(&make_response_frame(0x01, 0xD0, 0x00, b"12345678\0"));
    let sn = device.get_serial_number().await.unwrap();
    assert_eq!(sn, "12345678");

    // Test get_article_code
    mock.push_rx_data(&make_response_frame(0x01, 0xD0, 0x00, b"1-100001-01\0"));
    let art = device.get_article_code().await.unwrap();
    assert_eq!(art, "1-100001-01");

    // Test get_product_subtype
    mock.push_rx_data(&make_response_frame(0x01, 0xD0, 0x00, &[0x05]));
    let sub = device.get_product_subtype().await.unwrap();
    assert_eq!(sub, 5);

    // Test get_version
    mock.push_rx_data(&make_response_frame(
        0x01,
        0xD1,
        0x00,
        &[0x03, 0x01, 0x00, 0x02, 0x00, 0x01, 0x00],
    ));
    let v = device.get_version().await.unwrap();
    assert_eq!(v.firmware.major, 3);
    assert_eq!(v.firmware.minor, 1);
    assert_eq!(v.hardware.major, 2);
    assert_eq!(v.hardware.minor, 0);
    assert_eq!(v.protocol.major, 1);
    assert_eq!(v.protocol.minor, 0);

    // Test get_error_state
    mock.push_rx_data(&make_response_frame(
        0x01,
        0xD2,
        0x00,
        &[0x00, 0x00, 0x00, 0x04, 0x02],
    ));
    let (state, last_err) = device.get_error_state(true).await.unwrap();
    assert_eq!(state, 4);
    assert_eq!(last_err, 2);

    // Test get_slave_address
    mock.push_rx_data(&make_response_frame(0x01, 0x90, 0x00, &[0x01]));
    let addr = device.get_slave_address().await.unwrap();
    assert_eq!(addr, 1);

    // Test set_slave_address
    mock.push_rx_data(&make_response_frame(0x01, 0x90, 0x00, &[]));
    device.set_slave_address(0x02, true).await.unwrap();
    assert_eq!(device.slave_address(), 2);

    // Test get_baudrate
    mock.push_rx_data(&make_response_frame(
        0x02,
        0x91,
        0x00,
        &[0x00, 0x01, 0xC2, 0x00],
    ));
    let baud = device.get_baudrate().await.unwrap();
    assert_eq!(baud, 115200);

    // Test get_reply_delay
    mock.push_rx_data(&make_response_frame(0x02, 0x95, 0x00, &[0x00, 0x32]));
    let delay = device.get_reply_delay().await.unwrap();
    assert_eq!(delay, 50);

    // Test get_system_up_time
    mock.push_rx_data(&make_response_frame(
        0x02,
        0x93,
        0x00,
        &[0x00, 0x00, 0x04, 0xD2],
    ));
    let uptime = device.get_system_up_time().await.unwrap();
    assert_eq!(uptime, 1234);

    // Test device_reset
    mock.push_rx_data(&make_response_frame(0x02, 0xD3, 0x00, &[]));
    device.device_reset().await.unwrap();

    // Test factory_reset
    mock.push_rx_data(&make_response_frame(0x02, 0x92, 0x00, &[]));
    device.factory_reset().await.unwrap();

    // Test device error response code (e.g., code 2 = UnknownCommand)
    mock.push_rx_data(&make_response_frame(0x02, 0xAA, 0x02, &[]));
    let raw_cmd = RawShdlcCommand::new(0xAA, &[], std::time::Duration::from_millis(100));
    let err = device.execute_raw(&raw_cmd).await.unwrap_err();
    match err {
        ShdlcError::DeviceError { code, message } => {
            assert_eq!(code, 2);
            assert!(message.contains("Unknown command"));
        }
        _ => panic!("Expected DeviceError"),
    }
}
