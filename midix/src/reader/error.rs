use super::Reader;
use crate::{ChunkError, ParseError};
use thiserror::Error;

#[doc = r#"
A set of errors that can occur while reading something into a midi representation
"#]
#[derive(Debug, Error)]
#[error("Reading at Position {position}, {kind}")]
pub struct ReaderError {
    position: usize,
    pub(crate) kind: ReaderErrorKind,
}

/// A kind of error that a reader can produce
#[derive(Debug, Error)]
pub enum ReaderErrorKind {
    /// Parsing errors
    #[error("Parsing {0}")]
    ParseError(#[from] ParseError),
    /// Errors unrelated to parsing (out of bounds)
    #[error("Reading {0}")]
    ReadError(#[from] ReadError),
}

impl ReaderErrorKind {
    pub(crate) fn chunk(chunk_err: ChunkError) -> Self {
        Self::ParseError(ParseError::Chunk(chunk_err))
    }
}

/// Errors reading from some [`MidiSource`](crate::prelude::MidiSource)
#[derive(Debug, Error)]
pub enum ReadError {
    /// Read out of bounds
    #[error("Read out of bounds!")]
    OutOfBounds,
}

impl ReaderError {
    /// Create a reader error from a position and kind
    pub fn new(position: usize, kind: ReaderErrorKind) -> Self {
        Self { position, kind }
    }
    /// True if out of bounds or unexpected end of file
    pub fn is_out_of_bounds(&self) -> bool {
        matches!(
            self.kind,
            ReaderErrorKind::ReadError(ReadError::OutOfBounds)
        )
    }

    /// Create a new invalid data error
    pub fn parse_error(position: usize, error: ParseError) -> Self {
        Self {
            position,
            kind: ReaderErrorKind::ParseError(error),
        }
    }

    /// Create a new out of bounds error
    pub fn oob(position: usize) -> Self {
        Self {
            position,
            kind: ReaderErrorKind::ReadError(ReadError::OutOfBounds),
        }
    }
}

/// A result type that is either `T` or an [`io::Error`].
///
/// This may change in a future release if `midix`
/// should support `no-std` environments.
pub type ReadResult<T> = Result<T, ReaderError>;

// pub(crate) fn unexp_eof() -> ReaderError {
//     io::Error::new(ErrorKind::UnexpectedEof, "Read past the end of the file").into()
// }

pub(crate) fn inv_data<R>(reader: &mut Reader<R>, v: impl Into<ParseError>) -> ReaderError {
    reader.set_last_error_offset(reader.buffer_position());
    ReaderError::parse_error(reader.buffer_position(), v.into())
}
// #[allow(dead_code)]
// pub(crate) fn inv_input<R>(reader: &mut Reader<R>, v: impl fmt::Display) -> ReaderError {
//     reader.set_last_error_offset(reader.buffer_position());
//     io::Error::new(
//         ErrorKind::InvalidInput,
//         format!("Cursor at {}: {}", reader.buffer_position(), v),
//     )
//     .into()
// }
