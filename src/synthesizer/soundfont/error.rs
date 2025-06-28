use core::fmt::{self, Display, Formatter, Write};
use std::{error, io, string::String};

/// Represents an error when loading a SoundFont.
#[derive(Debug)]
pub enum SoundFontError {
    IoError(io::Error),
    RiffChunkNotFound,
    InvalidRiffChunkType {
        expected: FourCC,
        actual: FourCC,
    },
    ListChunkNotFound,
    InvalidListChunkType {
        expected: FourCC,
        actual: FourCC,
    },
    ListContainsUnknownId(FourCC),
    SampleDataNotFound,
    UnsupportedSampleFormat,
    SubChunkNotFound(FourCC),
    InvalidPresetList,
    InvalidInstrumentId {
        preset_id: usize,
        instrument_id: usize,
    },
    InvalidPreset(usize),
    PresetNotFound,
    InvalidInstrumentList,
    InvalidSampleId {
        instrument_id: usize,
        sample_id: usize,
    },
    InvalidInstrument(usize),
    InstrumentNotFound,
    InvalidSampleHeaderList,
    InvalidZoneList,
    ZoneNotFound,
    InvalidGeneratorList,
    RegionCheckFailed {
        inst_name: String,
        region_idx: usize,
        msg: String,
    },

    RegionSampleOutOfBounds {
        inst_name: String,
        region_idx: usize,
    },
}

impl error::Error for SoundFontError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SoundFontError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for SoundFontError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SoundFontError::IoError(err) => Display::fmt(&err, f),
            SoundFontError::RiffChunkNotFound => write!(f, "the RIFF chunk was not found"),
            SoundFontError::InvalidRiffChunkType { expected, actual } => write!(
                f,
                "the type of the RIFF chunk must be '{expected}', but was '{actual}'",
            ),
            SoundFontError::ListChunkNotFound => write!(f, "the LIST chunk was not found"),
            SoundFontError::InvalidListChunkType { expected, actual } => write!(
                f,
                "the type of the LIST chunk must be '{expected}', but was '{actual}'",
            ),
            SoundFontError::ListContainsUnknownId(id) => {
                write!(f, "the INFO list contains an unknown ID '{id}'")
            }
            SoundFontError::SampleDataNotFound => write!(f, "no valid sample data was found"),
            SoundFontError::UnsupportedSampleFormat => write!(f, "SoundFont3 is not yet supported"),
            SoundFontError::SubChunkNotFound(id) => {
                write!(f, "the '{id}' sub-chunk was not found")
            }
            SoundFontError::InvalidPresetList => write!(f, "the preset list is invalid"),
            SoundFontError::InvalidInstrumentId {
                preset_id,
                instrument_id,
            } => write!(
                f,
                "the preset with the ID '{preset_id}' contains an invalid instrument ID '{instrument_id}'"
            ),
            SoundFontError::InvalidPreset(preset_id) => {
                write!(f, "the preset with the ID '{preset_id}' has no zone")
            }
            SoundFontError::PresetNotFound => write!(f, "no valid preset was found"),
            SoundFontError::InvalidInstrumentList => write!(f, "the instrument list is invalid"),
            SoundFontError::InvalidSampleId {
                instrument_id,
                sample_id,
            } => write!(
                f,
                "the instrument with the ID '{instrument_id}' contains an invalid sample ID '{sample_id}'"
            ),
            SoundFontError::InvalidInstrument(instrument_id) => {
                write!(
                    f,
                    "the instrument with the ID '{instrument_id}' has no zone"
                )
            }
            SoundFontError::InstrumentNotFound => write!(f, "no valid instrument was found"),
            SoundFontError::InvalidSampleHeaderList => {
                write!(f, "the sample header list is invalid")
            }
            SoundFontError::InvalidZoneList => write!(f, "the zone list is invalid"),
            SoundFontError::ZoneNotFound => write!(f, "no valid zone was found"),
            SoundFontError::InvalidGeneratorList => write!(f, "the generator list is invalid"),
            SoundFontError::RegionCheckFailed {
                inst_name,
                region_idx,
                msg,
            } => {
                write!(f, "Error at inst {inst_name}, zone {region_idx}: {msg}")
            }
            SoundFontError::RegionSampleOutOfBounds {
                inst_name,
                region_idx,
            } => {
                write!(
                    f,
                    "Error at inst {inst_name}, zone {region_idx}: Sample out of bounds"
                )
            }
        }
    }
}

impl From<io::Error> for SoundFontError {
    fn from(err: io::Error) -> Self {
        SoundFontError::IoError(err)
    }
}

/// Reperesents the FourCC.
/// This is used for error reporting when the binary format is invalid.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct FourCC([u8; 4]);

impl FourCC {
    pub(crate) const fn from_bytes(mut bytes: [u8; 4]) -> Self {
        // Replace non-ASCII characters with '?'.
        bytes[0] = replace_with_question_mark_if_non_ascii(bytes[0]);
        bytes[1] = replace_with_question_mark_if_non_ascii(bytes[1]);
        bytes[2] = replace_with_question_mark_if_non_ascii(bytes[2]);
        bytes[3] = replace_with_question_mark_if_non_ascii(bytes[3]);
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; 4] {
        &self.0
    }
}

impl Display for FourCC {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for byte in &self.0 {
            f.write_char(*byte as char)?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for FourCC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('"')?;
        for byte in &self.0 {
            f.write_char(*byte as char)?;
        }
        f.write_char('"')?;
        Ok(())
    }
}

impl PartialEq<&[u8; 4]> for FourCC {
    fn eq(&self, other: &&[u8; 4]) -> bool {
        &self.0 == *other
    }
}

impl PartialEq<[u8; 4]> for FourCC {
    fn eq(&self, other: &[u8; 4]) -> bool {
        &self.0 == other
    }
}

const fn is_ascii_graphic_or_space(byte: u8) -> bool {
    byte.is_ascii_graphic() || byte == b' '
}

const fn replace_with_question_mark_if_non_ascii(byte: u8) -> u8 {
    if is_ascii_graphic_or_space(byte) {
        byte
    } else {
        b'?'
    }
}
