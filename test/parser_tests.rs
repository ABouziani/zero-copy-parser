use zero_copy_parser::parser::parse_packet;

#[test]
fn parses_packet() {
    let data = [
        1,
        0b0000_0011,
        0x00,
        0x05,
        b'H',
        b'e',
        b'l',
        b'l',
        b'o',
    ];

    let packet = parse_packet(&data).unwrap();

    assert_eq!(packet.version, 1);
    assert_eq!(packet.flags, 3);
    assert_eq!(packet.length, 5);
    assert_eq!(packet.payload, b"Hello");
}

#[test]
fn rejects_truncated_header() {
    let data = [1, 2, 0];

    assert!(parse_packet(&data).is_err());
}

#[test]
fn rejects_truncated_payload() {
    let data = [
        1,
        0,
        0,
        5,
        b'H',
        b'e',
    ];

    assert!(parse_packet(&data).is_err());
}

#[test]
fn payload_is_zero_copy() {
    let data = [
        1,
        0,
        0,
        5,
        b'H',
        b'e',
        b'l',
        b'l',
        b'o',
    ];

    let packet = parse_packet(&data).unwrap();

    assert_eq!(
        packet.payload.as_ptr(),
        unsafe { data.as_ptr().add(4) }
    );
}