use crate::prelude::*;

mod chunk;
pub use chunk::*;

#[doc = r#"
An event that can be yielded from or put into a `.mid` file.

This type is yielded by [`Reader::read_event`] and will be consumed by a Writer in the future.

# Overview

Except [`FileEvent::EOF`] Events can be placed into two categories

## Chunk Events

Three events, [`FileEvent::Header`], [`FileEvent::Track`], and [`FileEvent::Unknown`]
will contain meta information about a MIDI chunk.

- A [`FileEvent::Header`] is a header chunk that provides a minimal amount
of information pertaining to the entire MIDI file. Headers have no "bodies" and
therefore there is no `FileEvent::HeaderEvent` type.

- A [`FileEvent::Track`] is the metadata of a track chunk.
It contains the total length of the proceeding events.
    - The concepts of multiple tracks, multiple MIDI outputs, patterns, sequences,
    and songs may all be implemented using several track chunks given that the [`FormatType`]
    of the midi file is not [`FormatType::SingleMultiChannel`].

- An 'unknown' chunk begins with a chunk name that is not `MThd` or `MTrk`.
The [`Reader`] leaves it up to the user on how to handle these chunks.



## Track Events

A [`FileEvent::TrackEvent`] identifies some event yielded in the
body of a MIDI `track` chunk. This is usually sequential stream
of MIDI data which may contain information for up to 16 MIDI channels.
"#]
#[derive(Clone, Debug, PartialEq)]
pub enum FileEvent<'a> {
    /// A midi header
    ///
    /// See [`RawHeaderChunk`] for a breakdown on layout
    Header(RawHeaderChunk),

    /// A track chunk header
    ///
    /// See [`TrackChunkHeader`] for a breakdown on layout
    Track(TrackChunkHeader),

    /// An unknown event.
    ///
    /// This assumes a 4-byte name and a proceeding 4-byte length.
    ///
    /// Be wary of invalid chunks! This can lead to confusing
    /// behavior for proceeding events.
    Unknown(UnknownChunk<'a>),

    /// A track event.
    ///
    /// See [`TrackEvent`] for a detailed breakdown
    TrackEvent(TrackEvent<'a>),

    /// Yielded when no more bytes can be read
    EOF,
}

impl<'a> From<RawHeaderChunk> for FileEvent<'a> {
    fn from(value: RawHeaderChunk) -> Self {
        Self::Header(value)
    }
}

impl From<TrackChunkHeader> for FileEvent<'_> {
    fn from(value: TrackChunkHeader) -> Self {
        Self::Track(value)
    }
}

impl<'a> From<TrackEvent<'a>> for FileEvent<'a> {
    fn from(value: TrackEvent<'a>) -> Self {
        Self::TrackEvent(value)
    }
}
