use std::{
    fmt,
    io::{self, ErrorKind},
};
use thiserror::Error;

use super::Reader;

#[doc = r#"
A set of errors that can occur while reading something into a midi representation
"#]
#[derive(Error, Debug)]
pub enum ReaderError {
    /// Some io::Error
    #[error("{0}")]
    Io(#[from] io::Error),

    /// Unexpected out of bounds. Not necessarily IO related
    #[error("Out of bounds error: {0}")]
    OutOfBounds(String),

    /// Some Unknown error.
    #[error("Unknown error occurred")]
    Unknown,
}

impl ReaderError {
    /// True if out of bounds or unexpected end of file
    pub fn is_out_of_bounds(&self) -> bool {
        match self {
            Self::Io(o) => o.kind() == ErrorKind::UnexpectedEof,
            Self::OutOfBounds(_) => true,
            _ => false,
        }
    }

    /// Create a new invalid data error
    pub fn invalid_data(msg: impl fmt::Display) -> Self {
        Self::Io(io::Error::new(ErrorKind::InvalidData, msg.to_string()))
    }

    /// Create a new out of bounds error
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
