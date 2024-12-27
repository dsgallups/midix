use std::{
    fmt,
    io::{self, ErrorKind},
};

use thiserror::Error;

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

pub(super) fn unexp_eof() -> io::Error {
    io::Error::new(ErrorKind::UnexpectedEof, "Read past the end of the file")
}

pub(super) fn inv_data(pos: usize, v: impl fmt::Display) -> io::Error {
    io::Error::new(ErrorKind::InvalidData, format!("Cursor at {}: {}", pos, v))
}
pub(super) fn inv_input(pos: usize, v: impl fmt::Display) -> io::Error {
    io::Error::new(ErrorKind::InvalidInput, format!("Cursor at {}: {}", pos, v))
}
