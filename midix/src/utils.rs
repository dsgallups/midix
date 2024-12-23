use std::io::ErrorKind;

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
