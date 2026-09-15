# -*- coding: utf-8 -*-
from rust_shdlc_driver import ShdlcMockPort


def test_mock_port_rw():
    port = ShdlcMockPort(bitrate=9600)
    assert port.bitrate == 9600
    assert "MockTransport" in port.description
    assert port.is_open is True

    port.push_rx_data(b"\x7e\x00\x00\x00\x00\xff\x7e")
    addr, cmd, state, data = port.transceive(0x00, 0x00, b"", 0.1)
    assert addr == 0
    assert cmd == 0
    assert state == 0
    assert data == b""

    written = port.get_written_data()
    assert len(written) == 1
    assert written[0] == b"\x7e\x00\x00\x00\xff\x7e"
