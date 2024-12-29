use crate::prelude::*;

#[doc = r#"

An event that can be yielded from or put into a `.mid` file.

# Overview

Except [`FileEvent::EOF`], Events can be placed into two categories

## Chunk Events

MIDI Files contain two types of chunks: header chunks and
track chunks.
- A [`FileEvent::Header`] chunk provides a minimal amount
of information pertaining to the entire MIDI file.
- A `track` chunk contains a sequential stream of MIDI
data which may contain information for up to 16 MIDI channels.

The concepts of multiple tracks, multiple MIDI outputs, patterns, sequences,
and songs may all be implemented using several track chunks.

- An 'unknown' chunk begins with a chunk name that is not `MThd` or `MTrk`.
The [`Reader`] leaves it up to the user on how to handle these chunks.

Three events, `FileEvent::Header`, `FileEvent::Track`, and `FileEvent::Unknown`
will contain meta information about a MIDI chunk:

## Track Events

A [`FileEvent::TrackEvent`] identifies some event yielded in the body of a MIDI `track` chunk.




"#]
#[derive(Clone, Debug, PartialEq)]
pub enum FileEvent<'a> {
    /// A header event.
    ///
    /// See [`HeaderChunk`] for a breakdown on layout
    Header(HeaderChunk<'a>),

    /// A track event
    ///
    /// See [`TrackChunk`] for a breakdown on layout
    Track(TrackChunk),

    /// An unknown event.
    ///
    /// This assumes a 4-byte name and a proceeding 4-byte length
    Unknown(UnknownChunk<'a>),

    /// A track event.
    ///
    /// See [`TrackEvent`] for a detailed breakdown
    TrackEvent(TrackEvent<'a>),

    /// Yielded when no more bytes can be read
    EOF,
}

impl<'a> FileEvent<'a> {
    /// Create a new file event from a [`HeaderChunk`]
    pub fn header(h: HeaderChunk<'a>) -> Self {
        Self::Header(h)
    }

    /// Create a new file event from a [`TrackChunk`]
    pub fn track(t: TrackChunk) -> Self {
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
