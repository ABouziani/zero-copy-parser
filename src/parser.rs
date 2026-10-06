use crate::cursor::Cursor;

#[derive(Debug, PartialEq)]
pub struct Packet<'a> {
    pub version: u8,
    pub flags: u8,
    pub length: u16,
    pub payload: &'a [u8],
}

pub fn parse_packet(data: &[u8]) -> Result<Packet<'_>, &'static str> {
    let mut cursor = Cursor::new(data);

    let version = cursor.read_u8()?;
    let flags = cursor.read_u8()?;
    let length = cursor.read_u16_be()?;
    let payload = cursor.read_bytes(length as usize)?;

    Ok(Packet {
        version,
        flags,
        length,
        payload,
    })
}