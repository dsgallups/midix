use crate::prelude::*;

#[doc = r#"
Reads the full length of all chunk types


This is different from [`FileEvent`] such that
[`FileEvent::TrackEvent`] is not used. Instead,
the full set of bytes from the identified track are yielded.
"#]
pub enum ChunkEvent<'a> {
    /// A midi header
    ///
    /// See [`RawHeaderChunk`] for a breakdown on layout
    Header(RawHeaderChunk),
    /// The full bytes identified by a track chunk
    ///
    /// See [`RawTrackChunk`] for a breakdown on layout
    Track(RawTrackChunk<'a>),

    /// Some unknown type.
    ///
    /// See [`UnknownChunk`] for a breakdown on layout
    Unknown(UnknownChunk<'a>),
    /// End of File
    EOF,
}

impl From<RawHeaderChunk> for ChunkEvent<'_> {
    fn from(value: RawHeaderChunk) -> Self {
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

impl ChunkEvent<'_> {
    /// True if the event is the end of a file
    #[inline]
    pub const fn is_eof(&self) -> bool {
        matches!(self, Self::EOF)
    }
}
