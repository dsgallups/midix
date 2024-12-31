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
    /// Create a new [`FileEvent::Header`] from a [`RawHeaderChunk`]
    pub fn header(h: RawHeaderChunk<'a>) -> Self {
        Self::Header(h)
    }

    /// Create a new [`FileEvent::Track`] from a [`TrackChunkHeader`]
    pub fn track(t: RawTrackChunk<'a>) -> Self {
        Self::Track(t)
    }

    /// Create a new [`FileEvent::TrackEvent`] from a [`TrackEvent`]
    pub fn unknown(t: UnknownChunk<'a>) -> Self {
        Self::Unknown(t)
    }

    /// Create an [`FileEvent::EOF`] event
    pub fn eof() -> Self {
        Self::EOF
    }
}
