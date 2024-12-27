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
#[cfg(test)]
pub fn read_u32(reader: &mut OldReader<&[u8]>) -> OldReadResult<u32> {
    let chunk_size: &[u8; 4] = reader.read_exact_size()?;
    // this takes some time but like, it's pretty fast
    Ok(u32::from_be_bytes(*chunk_size))
}
#[cfg(test)]
pub fn read_u16(reader: &mut OldReader<&[u8]>) -> OldReadResult<u16> {
    let chunk_size: &[u8; 2] = reader.read_exact_size()?;
    // this takes some time but like, it's pretty fast
    Ok(u16::from_be_bytes(*chunk_size))
}

pub fn convert_u32(bytes: &[u8; 4]) -> u32 {
    u32::from_be_bytes(*bytes)
}

pub fn decode_varlen(reader: &mut OldReader<&[u8]>) -> OldReadResult<u32> {
    let mut dec: u32 = 0;

    for _ in 0..4 {
        let next = reader.read_next()?;
        dec <<= 7;
        let add = (next & 0x7F) as u32;
        dec |= add;

        //need to continue
        if next & 0x80 != 0x80 {
            break;
        }
    }

    Ok(dec)
}

#[test]
fn test_varlen_decode_simple() {
    let val = [0x03];
    let mut reader = OldReader::from_byte_slice(&val);
    let res = decode_varlen(&mut reader).unwrap();
    assert_eq!(res, 3);
}

#[test]
fn test_varlen_decode_combined() {
    //10010000 00001000
    let val = [0x90, 0x08];
    let mut reader = OldReader::from_byte_slice(&val);

    //should be 00100000001000
    //which is 2056
    let res = decode_varlen(&mut reader).unwrap();
    assert_eq!(res, 2056);

    let val = [0x81, 0x48];
    let mut reader = OldReader::from_byte_slice(&val);
    let res = decode_varlen(&mut reader).unwrap();
    assert_eq!(res, 200);
}

#[test]
fn test_long_varlen() {
    //11111111 10010001 10010000 00001000
    let val = [0xFF, 0x91, 0x90, 0x08];
    let mut reader = OldReader::from_byte_slice(&val);

    //should be 1111111001000100100000001000
    //which is 266618888
    let res = decode_varlen(&mut reader).unwrap();
    assert_eq!(res, 266618888);
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
    let mut reader = OldReader::from_byte_slice(&bytes);

    utils::read_u32(&mut reader).unwrap();
    utils::read_u16(&mut reader).unwrap();
    utils::read_u16(&mut reader).unwrap();
    utils::read_u16(&mut reader).unwrap();

    assert_eq!(reader.buffer_position(), 10);
}
