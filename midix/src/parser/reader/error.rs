use std::{
    fmt,
    io::{self, ErrorKind},
};

use thiserror::Error;

use super::Reader;

pub type ReadResult<T> = Result<T, io::Error>;

#[derive(Error, Debug)]
pub(crate) enum ParserError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("End of Reader")]
    EndOfReader,
    #[error("This MIDI file is unsupported: {0}")]
    Unimplemented(String),
}

pub type ParseResult<T> = Result<T, ParserError>;

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
pub(crate) fn inv_input<R>(reader: &mut Reader<R>, v: impl fmt::Display) -> io::Error {
    reader.set_last_error_offset(reader.buffer_position());
    io::Error::new(
        ErrorKind::InvalidInput,
        format!("Cursor at {}: {}", reader.buffer_position(), v),
    )
}
