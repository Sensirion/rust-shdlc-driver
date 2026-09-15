use rust_shdlc_driver::protocol::*;

#[test]
fn test_version_display() {
    let fv = FirmwareVersion::new(1, 2, false);
    assert_eq!(fv.to_string(), "1.2");
    let fv_debug = FirmwareVersion::new(2, 5, true);
    assert_eq!(fv_debug.to_string(), "2.5-debug");

    let hv = HardwareVersion::new(3, 0);
    assert_eq!(hv.to_string(), "3.0");

    let pv = ProtocolVersion::new(1, 0);
    assert_eq!(pv.to_string(), "1.0");

    let v = Version::new(fv, hv, pv);
    assert_eq!(v.to_string(), "Firmware 1.2, Hardware 3.0, Protocol 1.0");
}

#[test]
fn test_mosi_framing_cases() {
    // All zeros no data
    let f1 = ShdlcMosiFrame::new(0x00, 0x00, &[]);
    assert_eq!(f1.to_bytes(), b"\x7e\x00\x00\x00\xff\x7e");

    // All zeros 255 bytes
    let zeros_255 = vec![0x00; 255];
    let f2 = ShdlcMosiFrame::new(0x00, 0x00, &zeros_255);
    let mut expected2 = vec![0x7e, 0x00, 0x00, 0xff];
    expected2.extend_from_slice(&zeros_255);
    expected2.extend_from_slice(&[0x00, 0x7e]);
    assert_eq!(f2.to_bytes(), expected2);

    // Byte stuffing in address, command and data
    let f3 = ShdlcMosiFrame::new(0x7e, 0x7d, &[0x11, 0x12, 0x13, 0x14]);
    assert_eq!(
        f3.to_bytes(),
        b"\x7e\x7d\x5e\x7d\x5d\x04\x7d\x31\x12\x7d\x33\x14\xb6\x7e"
    );
}

#[test]
fn test_miso_decoding_cases() {
    let mut builder = ShdlcMisoFrameBuilder::new();
    let complete = builder.add_data(b"\x7e\x00\x00\x00\x00\xff\x7e").unwrap();
    assert!(complete);
    let frame = builder.interpret_data().unwrap();
    assert_eq!(frame.slave_address, 0x00);
    assert_eq!(frame.command_id, 0x00);
    assert_eq!(frame.state, 0x00);
    assert!(!frame.is_error_state());
    assert_eq!(frame.error_code(), 0);
    assert!(frame.data.is_empty());
}

#[test]
fn test_device_info_commands() {
    let cmd = GetProductType;
    let resp = cmd.interpret_response(b"00000001\0").unwrap();
    assert_eq!(resp, "00000001");

    let cmd_sub = GetProductSubType;
    let resp_sub = cmd_sub.interpret_response(&[0x02]).unwrap();
    assert_eq!(resp_sub, 2);
}

#[test]
fn test_version_command() {
    let cmd = GetVersion;
    let resp = cmd
        .interpret_response(&[0x01, 0x02, 0x00, 0x03, 0x04, 0x01, 0x00])
        .unwrap();
    assert_eq!(resp.firmware.major, 1);
    assert_eq!(resp.firmware.minor, 2);
    assert!(!resp.firmware.debug);
    assert_eq!(resp.hardware.major, 3);
    assert_eq!(resp.hardware.minor, 4);
    assert_eq!(resp.protocol.major, 1);
    assert_eq!(resp.protocol.minor, 0);
}

#[test]
fn test_error_state_command() {
    let cmd = GetErrorState::new(true);
    let resp = cmd
        .interpret_response(&[0x00, 0x00, 0x00, 0x10, 0x03])
        .unwrap();
    assert_eq!(resp.0, 16);
    assert_eq!(resp.1, 3);
}

#[test]
fn test_baudrate_command() {
    let set_cmd = SetBaudrate::new(115200);
    assert_eq!(set_cmd.data(), vec![0x00, 0x01, 0xC2, 0x00]);

    let get_cmd = GetBaudrate;
    let baud = get_cmd
        .interpret_response(&[0x00, 0x01, 0xC2, 0x00])
        .unwrap();
    assert_eq!(baud, 115200);
}
