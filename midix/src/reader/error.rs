use std::{
    fmt,
    io::{self, ErrorKind},
};
use thiserror::Error;

use super::Reader;

#[derive(Error, Debug)]
pub enum ReaderError {
    #[error("{0}")]
    Io(#[from] io::Error),

    #[error("Out of bounds error: {0}")]
    OutOfBounds(String),
    #[error("Unknown error occurred")]
    Unknown,
}

impl ReaderError {
    pub fn is_eof(&self) -> bool {
        match self {
            Self::Io(o) => o.kind() == ErrorKind::UnexpectedEof,
            _ => false,
        }
    }
    pub fn invalid_data(msg: impl fmt::Display) -> Self {
        Self::Io(io::Error::new(ErrorKind::InvalidData, msg.to_string()))
    }
    pub fn oob(msg: impl fmt::Display) -> Self {
        Self::OutOfBounds(msg.to_string())
    }
}

/// A result type that is either `T` or an [`io::Error`].
///
/// This may change in a future release if `midix`
/// should support `no-std` environments.
pub type ReadResult<T> = Result<T, ReaderError>;

pub(crate) fn unexp_eof() -> ReaderError {
    io::Error::new(ErrorKind::UnexpectedEof, "Read past the end of the file").into()
}

pub(crate) fn inv_data<R>(reader: &mut Reader<R>, v: impl fmt::Display) -> ReaderError {
    reader.set_last_error_offset(reader.buffer_position());
    io::Error::new(
        ErrorKind::InvalidData,
        format!("Cursor at {}: {}", reader.buffer_position(), v),
    )
    .into()
}
#[allow(dead_code)]
pub(crate) fn inv_input<R>(reader: &mut Reader<R>, v: impl fmt::Display) -> ReaderError {
    reader.set_last_error_offset(reader.buffer_position());
    io::Error::new(
        ErrorKind::InvalidInput,
        format!("Cursor at {}: {}", reader.buffer_position(), v),
    )
    .into()
}
