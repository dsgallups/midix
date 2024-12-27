use std::{
    fmt,
    io::{self, ErrorKind},
};

use super::Reader;

pub type ReadResult<T> = Result<T, io::Error>;

pub(crate) fn unexp_eof() -> io::Error {
    io::Error::new(ErrorKind::UnexpectedEof, "Read past the end of the file")
}

pub(crate) fn inv_data<R>(reader: &mut Reader<R>, v: impl fmt::Display) -> io::Error {
    reader.set_last_error_offset(reader.buffer_position());
    io::Error::new(
        ErrorKind::InvalidData,
        format!("Cursor at {}: {}", reader.buffer_position(), v),
    )
}
#[allow(dead_code)]
pub(crate) fn inv_input<R>(reader: &mut Reader<R>, v: impl fmt::Display) -> io::Error {
    reader.set_last_error_offset(reader.buffer_position());
    io::Error::new(
        ErrorKind::InvalidInput,
        format!("Cursor at {}: {}", reader.buffer_position(), v),
    )
}
