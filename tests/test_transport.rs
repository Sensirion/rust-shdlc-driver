use rust_shdlc_driver::protocol::*;
use rust_shdlc_driver::transport::{MockTransport, ShdlcTransceiver, ShdlcTransport};
use std::time::Duration;

#[tokio::test]
async fn test_mock_transceiver_success() {
    let mut mock = MockTransport::new();

    // Prepare response frame: slave 0x00, cmd 0xD0, state 0x00, len 8, "00000001", checksum, framed
    // MOSI will be sent by transceiver.
    // Response frame bytes:
    // Raw frame: [0x7e, 0x00, 0xd0, 0x00, 0x08, b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'1', checksum, 0x7e]
    let mut content = vec![0x00, 0xD0, 0x00, 0x08];
    content.extend_from_slice(b"00000001");
    let checksum = calculate_checksum(&content);
    content.push(checksum);
    let stuffed = stuff_data_bytes(&content);
    let mut raw_response = vec![START_STOP_BYTE];
    raw_response.extend_from_slice(&stuffed);
    raw_response.push(START_STOP_BYTE);

    mock.push_rx_data(&raw_response);

    let miso = ShdlcTransceiver::transceive(
        &mut mock,
        0x00,
        0xD0,
        &[0x00],
        Duration::from_millis(100),
        Duration::from_millis(10),
    )
    .await
    .unwrap();

    assert_eq!(miso.slave_address, 0x00);
    assert_eq!(miso.command_id, 0xD0);
    assert_eq!(miso.state, 0x00);
    assert_eq!(miso.data, b"00000001");

    // Check that MOSI was transmitted to the mock
    let written = mock.get_written_data();
    assert_eq!(written.len(), 1);
    let mosi = ShdlcMosiFrame::new(0x00, 0xD0, &[0x00]);
    assert_eq!(written[0], mosi.to_bytes());
}

#[tokio::test]
async fn test_mock_transceiver_timeout() {
    let mut mock = MockTransport::new();
    mock.set_timeout_on_read(true);

    let res = ShdlcTransceiver::transceive(
        &mut mock,
        0x00,
        0x90,
        &[],
        Duration::from_millis(50),
        Duration::from_millis(10),
    )
    .await;

    assert!(matches!(res, Err(ShdlcError::Timeout)));
}

#[tokio::test]
async fn test_mock_transport_bitrate() {
    let mut mock = MockTransport::with_bitrate(9600);
    assert_eq!(mock.bitrate().unwrap(), 9600);
    mock.set_bitrate(115200).await.unwrap();
    assert_eq!(mock.bitrate().unwrap(), 115200);

    mock.set_support_bitrate_change(false);
    assert!(mock.set_bitrate(9600).await.is_err());
}
