use zero_copy_parser::cursor::Cursor;
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
fn parses_empty_payload() {
    let data = [
        1,
        0,
        0x00,
        0x00,
    ];

    let packet = parse_packet(&data).unwrap();

    assert_eq!(packet.version, 1);
    assert_eq!(packet.flags, 0);
    assert_eq!(packet.length, 0);
    assert!(packet.payload.is_empty());
}

#[test]
fn parses_binary_payload() {
    let data = [
        1,
        2,
        0x00,
        0x04,
        0x00,
        0xFF,
        0xAA,
        0x55,
    ];

    let packet = parse_packet(&data).unwrap();

    assert_eq!(packet.payload, &[0x00, 0xFF, 0xAA, 0x55]);
}

#[test]
fn parses_max_u16_payload_length() {
    let mut data = vec![1, 0, 0xFF, 0xFF];
    data.extend(std::iter::repeat_n(0xAA, 65535));

    let packet = parse_packet(&data).unwrap();

    assert_eq!(packet.length, 65535);
    assert_eq!(packet.payload.len(), 65535);
    assert_eq!(packet.payload[0], 0xAA);
    assert_eq!(packet.payload[65534], 0xAA);
}

#[test]
fn rejects_empty_input() {
    let data = [];

    assert!(parse_packet(&data).is_err());
}

#[test]
fn rejects_one_byte_header() {
    let data = [1];

    assert!(parse_packet(&data).is_err());
}

#[test]
fn rejects_two_byte_header() {
    let data = [1, 2];

    assert!(parse_packet(&data).is_err());
}

#[test]
fn rejects_three_byte_header() {
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
fn rejects_payload_short_by_one_byte() {
    let data = [
        1,
        0,
        0,
        3,
        b'A',
        b'B',
    ];

    assert!(parse_packet(&data).is_err());
}

#[test]
fn accepts_payload_exactly_matching_length() {
    let data = [
        1,
        0,
        0,
        3,
        b'A',
        b'B',
        b'C',
    ];

    let packet = parse_packet(&data).unwrap();

    assert_eq!(packet.payload, b"ABC");
}

#[test]
fn ignores_extra_data_after_payload() {
    let data = [
        1,
        0,
        0,
        3,
        b'A',
        b'B',
        b'C',
        0xFF,
        0xFF,
    ];

    let packet = parse_packet(&data).unwrap();

    assert_eq!(packet.payload, b"ABC");
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

    assert_eq!(packet.payload.as_ptr(), data[4..].as_ptr());
}

#[test]
fn payload_length_matches_declared_length() {
    let data = [
        1,
        3,
        0x00,
        0x04,
        10,
        20,
        30,
        40,
    ];

    let packet = parse_packet(&data).unwrap();

    assert_eq!(packet.payload.len(), packet.length as usize);
}

#[test]
fn parses_all_flag_bits() {
    let data = [
        1,
        0xFF,
        0x00,
        0x00,
    ];

    let packet = parse_packet(&data).unwrap();

    assert_eq!(packet.flags, 0xFF);
}

#[test]
fn parses_zero_flags() {
    let data = [
        1,
        0x00,
        0x00,
        0x00,
    ];

    let packet = parse_packet(&data).unwrap();

    assert_eq!(packet.flags, 0);
}

#[test]
fn cursor_reads_u8() {
    let data = [0x10, 0x20];

    let mut cursor = Cursor::new(&data);

    assert_eq!(cursor.read_u8().unwrap(), 0x10);
    assert_eq!(cursor.read_u8().unwrap(), 0x20);
}

#[test]
fn cursor_rejects_u8_at_end() {
    let data = [0x10];

    let mut cursor = Cursor::new(&data);

    assert_eq!(cursor.read_u8().unwrap(), 0x10);
    assert!(cursor.read_u8().is_err());
}

#[test]
fn cursor_reads_u16_big_endian() {
    let data = [0x01, 0x02];

    let mut cursor = Cursor::new(&data);

    assert_eq!(cursor.read_u16_be().unwrap(), 0x0102);
}

#[test]
fn cursor_reads_u16_big_endian_zero() {
    let data = [0x00, 0x00];

    let mut cursor = Cursor::new(&data);

    assert_eq!(cursor.read_u16_be().unwrap(), 0);
}

#[test]
fn cursor_reads_u16_big_endian_max() {
    let data = [0xFF, 0xFF];

    let mut cursor = Cursor::new(&data);

    assert_eq!(cursor.read_u16_be().unwrap(), 65535);
}

#[test]
fn cursor_rejects_incomplete_u16() {
    let data = [0x01];

    let mut cursor = Cursor::new(&data);

    assert!(cursor.read_u16_be().is_err());
}

#[test]
fn cursor_reads_u32_little_endian() {
    let data = [0x04, 0x03, 0x02, 0x01];

    let mut cursor = Cursor::new(&data);

    assert_eq!(cursor.read_u32_le().unwrap(), 0x01020304);
}

#[test]
fn cursor_reads_u32_little_endian_zero() {
    let data = [0x00, 0x00, 0x00, 0x00];

    let mut cursor = Cursor::new(&data);

    assert_eq!(cursor.read_u32_le().unwrap(), 0);
}

#[test]
fn cursor_reads_u32_little_endian_max() {
    let data = [0xFF, 0xFF, 0xFF, 0xFF];

    let mut cursor = Cursor::new(&data);

    assert_eq!(cursor.read_u32_le().unwrap(), u32::MAX);
}

#[test]
fn cursor_rejects_incomplete_u32() {
    let data = [0x01, 0x02, 0x03];

    let mut cursor = Cursor::new(&data);

    assert!(cursor.read_u32_le().is_err());
}

#[test]
fn cursor_reads_bytes() {
    let data = [1, 2, 3, 4, 5];

    let mut cursor = Cursor::new(&data);

    let bytes = cursor.read_bytes(3).unwrap();

    assert_eq!(bytes, &[1, 2, 3]);
}

#[test]
fn cursor_reads_zero_bytes() {
    let data = [1, 2, 3];

    let mut cursor = Cursor::new(&data);

    let bytes = cursor.read_bytes(0).unwrap();

    assert!(bytes.is_empty());
}

#[test]
fn cursor_rejects_too_many_bytes() {
    let data = [1, 2, 3];

    let mut cursor = Cursor::new(&data);

    assert!(cursor.read_bytes(4).is_err());
}

#[test]
fn cursor_position_starts_at_zero() {
    let data = [1, 2, 3];

    let cursor = Cursor::new(&data);

    assert_eq!(cursor.position(), 0);
}

#[test]
fn cursor_position_updates_after_reading() {
    let data = [1, 2, 3, 4];

    let mut cursor = Cursor::new(&data);

    cursor.read_u8().unwrap();

    assert_eq!(cursor.position(), 1);

    cursor.read_u16_be().unwrap();

    assert_eq!(cursor.position(), 3);
}

#[test]
fn cursor_remaining_starts_at_full_length() {
    let data = [1, 2, 3, 4];

    let cursor = Cursor::new(&data);

    assert_eq!(cursor.remaining(), 4);
}

#[test]
fn cursor_remaining_decreases_after_reading() {
    let data = [1, 2, 3, 4];

    let mut cursor = Cursor::new(&data);

    cursor.read_u8().unwrap();

    assert_eq!(cursor.remaining(), 3);

    cursor.read_u16_be().unwrap();

    assert_eq!(cursor.remaining(), 1);
}

#[test]
fn cursor_can_read_entire_buffer() {
    let data = [1, 2, 3, 4];

    let mut cursor = Cursor::new(&data);

    assert_eq!(cursor.read_bytes(4).unwrap(), &data);
    assert_eq!(cursor.position(), 4);
    assert_eq!(cursor.remaining(), 0);
}

#[test]
fn cursor_cannot_read_past_buffer() {
    let data = [1, 2, 3];

    let mut cursor = Cursor::new(&data);

    cursor.read_bytes(3).unwrap();

    assert!(cursor.read_u8().is_err());
}

#[test]
fn cursor_reads_multiple_values() {
    let data = [
        0x01,
        0x02,
        0x00,
        0x03,
        0xAA,
        0xBB,
        0xCC,
    ];

    let mut cursor = Cursor::new(&data);

    assert_eq!(cursor.read_u8().unwrap(), 1);
    assert_eq!(cursor.read_u8().unwrap(), 2);
    assert_eq!(cursor.read_u16_be().unwrap(), 3);
    assert_eq!(cursor.read_bytes(3).unwrap(), &[0xAA, 0xBB, 0xCC]);
    assert_eq!(cursor.remaining(), 0);
}