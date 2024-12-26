use std::io::ErrorKind;

use crate::prelude::*;

pub fn check_u7(byte: u8) -> Result<u8, std::io::Error> {
    (byte & 0b10000000 == 0)
        .then_some(byte)
        .ok_or(io_error!(ErrorKind::InvalidData, "Leading bit found"))
}

pub fn check_u4(byte: u8) -> Result<u8, std::io::Error> {
    (byte & 0b11110000 == 0)
        .then_some(byte)
        .ok_or(io_error!(ErrorKind::InvalidData, "Leading bit found"))
}
pub fn read_u32(reader: &mut Reader<&[u8]>) -> ReadResult<u32> {
    let chunk_size: &[u8; 4] = reader.read_exact_size()?;
    // this takes some time but like, it's pretty fast
    Ok(u32::from_be_bytes(*chunk_size))
}
pub fn read_u16(reader: &mut Reader<&[u8]>) -> ReadResult<u16> {
    let chunk_size: &[u8; 2] = reader.read_exact_size()?;
    // this takes some time but like, it's pretty fast
    Ok(u16::from_be_bytes(*chunk_size))
}
#[allow(dead_code)]
pub fn peak_u16(reader: &mut Reader<&[u8]>) -> ReadResult<u16> {
    let chunk_size: [u8; 2] = reader.peek_exact(2)?.try_into().unwrap();
    // this takes some time but like, it's pretty fast
    Ok(u16::from_be_bytes(chunk_size))
}

pub fn convert_u32(bytes: &[u8; 4]) -> u32 {
    u32::from_be_bytes(*bytes)
}

#[test]
fn test_read_exact() {
    use crate::utils;
    let bytes = [
        0x00, 0x00, 0x00, 0x06, //length
        0x00, 0x01, //format
        0x00, 0x03, //num_tracks
        0x00, 0x78, //timing
    ];
    let mut reader = Reader::from_byte_slice(&bytes);

    utils::read_u32(&mut reader).unwrap();
    utils::read_u16(&mut reader).unwrap();
    utils::read_u16(&mut reader).unwrap();
    utils::read_u16(&mut reader).unwrap();

    assert_eq!(reader.buffer_position(), 10);
}
