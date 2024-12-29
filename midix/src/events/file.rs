use crate::prelude::*;

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
    /// See [`HeaderChunk`] for a breakdown on layout
    Header(HeaderChunk<'a>),

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

impl<'a> FileEvent<'a> {
    /// Create a new [`FileEvent::Header`] from a [`HeaderChunk`]
    pub fn header(h: HeaderChunk<'a>) -> Self {
        Self::Header(h)
    }

    /// Create a new [`FileEvent::Track`] from a [`TrackChunkHeader`]
    pub fn track(t: TrackChunkHeader) -> Self {
        Self::Track(t)
    }

    /// Create a new [`FileEvent::TrackEvent`] from a [`TrackEvent`]
    pub fn track_event(t: TrackEvent<'a>) -> Self {
        Self::TrackEvent(t)
    }

    /// Create an [`FileEvent::EOF`] event
    pub fn eof() -> Self {
        Self::EOF
    }
}
