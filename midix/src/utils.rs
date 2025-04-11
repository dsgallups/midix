use std::io::ErrorKind;

pub(crate) fn check_u7(byte: u8) -> Result<u8, std::io::Error> {
    (byte & 0b1000_0000 == 0)
        .then_some(byte)
        .ok_or(io_error!(ErrorKind::InvalidData, "Leading bit found"))
}
