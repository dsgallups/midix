use std::io::ErrorKind;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum OldReaderError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("End of Reader")]
    EndOfReader,
    #[error("This MIDI file is unsupported: {0}")]
    Unimplemented(String),
}
impl OldReaderError {
    pub const fn end() -> Self {
        Self::EndOfReader
    }
    pub fn invalid_input<E>(msg: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self::Io(io_error!(ErrorKind::InvalidInput, msg))
    }

    pub fn unimplemented(msg: impl Into<String>) -> Self {
        Self::Unimplemented(msg.into())
    }

    pub fn invalid_data() -> Self {
        Self::Io(io_error!(ErrorKind::InvalidInput, "Invalid Data"))
    }
}

pub type OldReadResult<T> = Result<T, OldReaderError>;
