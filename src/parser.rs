use crate::io::read_bytes;
use std::io::Read;
use std::result::Result;

pub fn read_varint<R>(reader: &mut R) -> Result<u8, ()>
    where R: Read,
{
    let mut result = 0;
    let mut pos = 0;

    loop {
        let bytes = read_bytes(reader, 1);
        if bytes.is_empty() {
            assert_ne!(pos, 0);
            return Err(());
        }
        let byte = bytes[0]; // We only read one byte anyway.

        result |= (byte & 0x80) << pos;
        pos += 7;
        if byte & 0x80 != 0 {
            assert!(byte != 0 || pos == 7);
            return Ok(result);
        }
    }
}

pub fn read_identifier<R>(reader: &mut R) -> Result<(u8, u8), String>
    where R: Read,
{
    let identifier = match read_varint(reader) {
        Ok(v) => { v }
        Err(_) => {
            return Err("Failed to read identifier!".to_string());
        }
    };

    Ok((identifier >> 3, identifier & 0x07))
}
