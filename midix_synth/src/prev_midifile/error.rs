use std::error;
use std::fmt;
use std::io;

use crate::soundfont::FourCC;

/// Represents an error when loading a MIDI file.
#[derive(Debug)]
#[non_exhaustive]
pub enum MidiFileError {
    IoError(io::Error),
    InvalidChunkType { expected: FourCC, actual: FourCC },
    InvalidChunkData(FourCC),
    UnsupportedFormat(i16),
    InvalidTempoValue,
}

impl error::Error for MidiFileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MidiFileError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for MidiFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MidiFileError::IoError(err) => err.fmt(f),
            MidiFileError::InvalidChunkType { expected, actual } => write!(
                f,
                "the chunk type must be '{}', but was '{}'",
                expected, actual
            ),
            MidiFileError::InvalidChunkData(id) => write!(f, "the '{}' chunk has invalid data", id),
            MidiFileError::UnsupportedFormat(format) => {
                write!(f, "the format {} is not supported", format)
            }
            MidiFileError::InvalidTempoValue => write!(f, "failed to read the tempo value"),
        }
    }
}

impl From<io::Error> for MidiFileError {
    fn from(err: io::Error) -> Self {
        MidiFileError::IoError(err)
    }
}
