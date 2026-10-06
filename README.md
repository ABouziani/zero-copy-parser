# Zero-Copy Parser

A zero-copy binary packet parser written in Rust.

This project focuses on parsing binary data efficiently without unnecessary heap allocations or copies.

## Features

* Zero-copy `&[u8]` payloads
* Cursor-based parsing
* Lifetime-safe borrowed data
* Big-endian integer decoding
* Little-endian integer decoding
* Bounds checking
* Borrowed packet structures
* No external dependencies

## Packet Format

Each packet uses the following format:

```text
+---------+-------+--------+-------------+
| version | flags | length |   payload   |
|  1 byte | 1 byte| 2 bytes|   N bytes   |
+---------+-------+--------+-------------+
```

### Fields

* `version`: protocol version
* `flags`: bit flags
* `length`: payload length encoded as big-endian `u16`
* `payload`: raw bytes borrowed from the input buffer

Example:

```text
01 03 00 05 48 65 6C 6C 6F
```

Decoded as:

```text
version = 1
flags   = 3
length  = 5
payload = "Hello"
```

## Zero-Copy Design

The parser returns a borrowed payload:

```rust
pub struct Packet<'a> {
    pub version: u8,
    pub flags: u8,
    pub length: u16,
    pub payload: &'a [u8],
}
```

The payload points directly into the original input buffer.

No `Vec<u8>` is created for the payload.

```text
Original buffer
┌──────────────────────────────────────┐
│ header │       payload               │
└────────┴─────────────────────────────┘
         ▲
         │
         └── Packet::payload
```

This avoids an unnecessary allocation and memory copy.

## Cursor-Based Parsing

Parsing is performed using a cursor:

```rust
pub struct Cursor<'a> {
    data: &'a [u8],
    pos: usize,
}
```

The cursor keeps track of the current position while reading values from the input buffer.

Supported operations include:

```rust
read_u8()
read_u16_be()
read_u32_le()
read_bytes()
```

## Endianness

The parser demonstrates explicit byte-order handling.

Big-endian:

```rust
u16::from_be_bytes(...)
```

Little-endian:

```rust
u32::from_le_bytes(...)
```

For example:

```text
01 02 03 04
```

interpreted as big-endian:

```text
0x01020304
```

interpreted as little-endian:

```text
0x04030201
```

The byte order must match the protocol being parsed.

## Safety

The parser checks input boundaries before reading data.

Truncated packets return an error instead of panicking.

Examples:

```text
unexpected end of input
integer overflow
```

The parser itself does not require `unsafe` code.

## Project Structure

```text
zero-copy-parser/
├── Cargo.toml
├── LICENSE
├── README.md
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── cursor.rs
│   └── parser.rs
└── tests/
    └── parser_tests.rs
```

## Example

```rust
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
}
```

Output:

```text
version: 1
flags:   0b00000011
length:  5
payload: [72, 101, 108, 108, 111]
```

## Tests

Run the test suite:

```bash
cargo test
```

The tests cover:

* Valid packet parsing
* Truncated headers
* Truncated payloads
* Zero-copy payload access

## Run

Run the example:

```bash
cargo run
```

## Concepts

This project is designed to demonstrate practical Rust concepts used in systems and infrastructure development:

* Binary protocols
* Bytes and hexadecimal representation
* Endianness
* Memory layout
* Slices
* Borrowing
* Lifetimes
* Zero-copy parsing
* Cursor-based parsing
* Bounds checking
* Heap allocation avoidance

These concepts are particularly useful when working with network protocols, blockchain clients, high-performance systems, and binary serialization formats.

## License

MIT License
