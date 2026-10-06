use zero_copy_parser::parser::parse_packet;

fn main() {
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

    println!("version: {}", packet.version);
    println!("flags:   0b{:08b}", packet.flags);
    println!("length:  {}", packet.length);
    println!("payload: {:?}", packet.payload);
    println!("payload ptr: {:p}", packet.payload.as_ptr());
    println!("data ptr:    {:p}", data.as_ptr());
}