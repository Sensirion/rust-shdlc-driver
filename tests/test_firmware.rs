use rust_shdlc_driver::connection::AsyncShdlcConnection;
use rust_shdlc_driver::device::AsyncShdlcDevice;
use rust_shdlc_driver::firmware::{ShdlcFirmwareImage, ShdlcFirmwareUpdate};
use rust_shdlc_driver::protocol::errors::ShdlcError;
use rust_shdlc_driver::protocol::*;
use rust_shdlc_driver::transport::MockTransport;
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

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

#[test]
fn test_firmware_image_eks2() {
    let hex_content = fs::read_to_string("tests/data/Eks2_combined_V5.2.hex").unwrap();
    let img = ShdlcFirmwareImage::new(&hex_content, 0x08000000, 0x08004000, None, None).unwrap();

    assert_eq!(img.product_type(), 0x00060000);
    assert_eq!(img.bootloader_version().major, 0);
    assert_eq!(img.bootloader_version().minor, 4);
    assert_eq!(img.application_version().major, 5);
    assert_eq!(img.application_version().minor, 2);
    assert_eq!(img.size(), 75096);
    assert_eq!(img.checksum(), 0x88);
}

#[test]
fn test_firmware_image_stm32g0() {
    let hex_content = fs::read_to_string("tests/data/Stm32g0Firmware.hex").unwrap();
    let img = ShdlcFirmwareImage::new(
        &hex_content,
        0x08000000,
        0x08001000,
        Some(b"\x4b\x4f\x47\x4a\xa4\x74\xf4\xb4"),
        Some(0x200),
    )
    .unwrap();

    assert_eq!(img.product_type(), 0x00140000);
    assert_eq!(img.bootloader_version().major, 1);
    assert_eq!(img.bootloader_version().minor, 0);
    assert_eq!(img.application_version().major, 0);
    assert_eq!(img.application_version().minor, 1);
    assert_eq!(img.size(), 1600);
    assert_eq!(img.checksum(), 0x3A);
}

#[test]
fn test_firmware_invalid_signature() {
    let hex_content = fs::read_to_string("tests/data/Eks2_combined_V5.2.hex").unwrap();
    // Pass wrong app address to trigger signature mismatch
    let res = ShdlcFirmwareImage::new(&hex_content, 0x08000000, 0x08004001, None, None);
    assert!(matches!(
        res,
        Err(ShdlcError::FirmwareImageSignatureError(_))
    ));
}

#[tokio::test]
async fn test_firmware_update_flow() {
    let hex_content = fs::read_to_string("tests/data/Stm32g0Firmware.hex").unwrap();
    let img = ShdlcFirmwareImage::new(
        &hex_content,
        0x08000000,
        0x08001000,
        Some(b"\x4b\x4f\x47\x4a\xa4\x74\xf4\xb4"),
        Some(0x200),
    )
    .unwrap();

    let mock = MockTransport::new();

    // 1. GetProductType response from device (slave 0, cmd 0xD0, payload "00140000\0")
    let resp_prod = make_response_frame(0x00, 0xD0, 0x00, b"00140000\0");
    mock.push_rx_data(&resp_prod);

    // 2. EnterBootloader response (slave 0, cmd 0xF3)
    let resp_boot = make_response_frame(0x00, 0xF3, 0x00, &[]);
    mock.push_rx_data(&resp_boot);

    // 3. FirmwareUpdateStart response (slave 0, cmd 0xF3)
    let resp_start = make_response_frame(0x00, 0xF3, 0x00, &[]);
    mock.push_rx_data(&resp_start);

    // 4. Multiple FirmwareUpdateData responses: 1600 bytes / 254 = 7 blocks (6*254 + 76)
    for _ in 0..7 {
        let resp_data = make_response_frame(0x00, 0xF3, 0x00, &[]);
        mock.push_rx_data(&resp_data);
    }

    // 5. FirmwareUpdateStop response (slave 0, cmd 0xF3)
    let resp_stop = make_response_frame(0x00, 0xF3, 0x00, &[]);
    mock.push_rx_data(&resp_stop);

    let conn = AsyncShdlcConnection::new(Box::new(mock.clone()));
    let device = AsyncShdlcDevice::new(conn, 0x00);

    let mut updater = ShdlcFirmwareUpdate::new(device, img);
    let progress_count = Arc::new(AtomicUsize::new(0));
    let p_clone = Arc::clone(&progress_count);
    updater.set_progress_callback(move |_p| {
        p_clone.fetch_add(1, Ordering::SeqCst);
    });

    updater.execute(false).await.unwrap();

    assert!(progress_count.load(Ordering::SeqCst) >= 7);
}
