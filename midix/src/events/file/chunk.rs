#[doc = r#"
Reads the full length of all chunk types


This is different from [`FileEvent`] such that
[`FileEvent::TrackEvent`] is not used. Instead,
the full set of bytes from the identified track are yielded.
"#]
use crate::prelude::*;

pub enum ChunkEvent<'a> {
    /// A midi header
    ///
    /// See [`RawHeaderChunk`] for a breakdown on layout
    Header(RawHeaderChunk<'a>),
    Track(RawTrackChunk<'a>),
    Unknown(UnknownChunk<'a>),
    EOF,
}

impl<'a> From<RawHeaderChunk<'a>> for ChunkEvent<'a> {
    fn from(value: RawHeaderChunk<'a>) -> Self {
        Self::Header(value)
    }
}

impl<'a> From<RawTrackChunk<'a>> for ChunkEvent<'a> {
    fn from(value: RawTrackChunk<'a>) -> Self {
        Self::Track(value)
    }
}

impl<'a> From<UnknownChunk<'a>> for ChunkEvent<'a> {
    fn from(value: UnknownChunk<'a>) -> Self {
        Self::Unknown(value)
    }
}

impl<'a> ChunkEvent<'a> {
    pub fn is_eof(&self) -> bool {
        matches!(self, Self::EOF)
    }
}
