#![doc = r"
Contains a high-level interface for a pull-based MIDI file parser

See the [`Reader`] docs for more information

# Acknowledgments

Inspired by <https://docs.rs/quick-xml/latest/quick_xml/>
"]

mod error;
mod source;
mod state;
pub use error::*;
pub use source::*;
use state::{ParseState, ReaderState};

use crate::prelude::*;

#[doc = r#"
A MIDI event reader.

Consumes bytes and streams MIDI [`FileEvent`]s.

# Overview
MIDI Files are made up of -chunks-. Each chunk has a 4-character
type and a 32-bit length, which is the number of bytes in the chunk.
This structure allows future chunk types to be designed which may be
easily be ignored if encountered by a program written before the
chunk type is introduced.

Each chunk begins with a 4-character ASCII type. It is followed by a
32-bit length, most significant byte first (a length of 6 is stored
as 00 00 00 06). This length refers to the number of bytes of data
which follow: the eight bytes of type and length are not included.
Therefore, a chunk with a length of 6 would actually occupy 14 bytes
in the disk file.

# Shortcomings

This reader will be able to yield events from any type
that is [`Read`](std::io::Read) in a future minor release.

For now, construct a `Reader<&[u8]>` to gain access to [`Reader::read_event`].

# Common Pitfalls
This parser will not error if an unknown chunk type is found. It will assume
the unknown data has a 4-byte name and a proceeding 4-byte length. If this
is not true, then the cursor will fail on the next read event.

# Example
```rust
use midix::prelude::*;

let midi_header = [
    /* MIDI Header */
    0x4D, 0x54, 0x68, 0x64, // "MThdd"
    0x00, 0x00, 0x00, 0x06, // Chunk length (6)
    0x00, 0x00, // format 0
    0x00, 0x01, // one track
    0x00, 0x60  // 96 per quarter note
];

let mut reader = Reader::from_byte_slice(&midi_header);

// The first and only event will be the midi header
let Ok(FileEvent::Header(header)) = reader.read_event() else {
    panic!("Expected a header event");
};

// format 0 implies a single multi-channel file (only one track)
assert_eq!(header.format_type(), FormatType::SingleMultiChannel);

assert_eq!(
    header.timing().ticks_per_quarter_note(),
    Some(96)
);
```

"#]
#[derive(Clone)]
pub struct Reader<R> {
    reader: R,
    pub(crate) state: ReaderState,
}

impl<R> Reader<R> {
    /// Create a new reader.
    pub const fn new(reader: R) -> Self {
        Self {
            reader,
            state: ReaderState::default(),
        }
    }

    /// Consume self to grab the inner reader
    pub fn into_inner(self) -> R {
        self.reader
    }

    pub(crate) fn set_last_error_offset(&mut self, offset: usize) {
        self.state.set_last_error_offset(offset);
    }

    /// Grab the current position of the inner reader "cursor"
    pub const fn buffer_position(&self) -> usize {
        self.state.offset()
    }

    /// Gets a reference to the underlying reader
    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    /// Gets a mutable reference to the underlying reader
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.reader
    }
}

impl<'slc> Reader<&'slc [u8]> {
    /// Create a new [`Reader`] from a `&[u8]`. Only this type has the
    /// [`Reader::read_event`] method.
    #[must_use]
    pub const fn from_byte_slice(slice: &'slc [u8]) -> Self {
        Self {
            reader: slice,
            state: ReaderState::default(),
        }
    }
}

impl<'a> Reader<Bytes<'a>> {
    pub fn from_bytes<B: Into<Bytes<'a>>>(slice: B) -> Self {
        Self {
            reader: slice.into(),
            state: ReaderState::default(),
        }
    }
}

//internal implementations
impl<'slc, R: MidiSource<'slc>> Reader<R> {
    // Returns None if there's no bytes left to read
    pub(super) fn read_exact<'slf>(&'slf mut self, bytes: usize) -> ReadResult<Bytes<'slc>>
    where
        'slc: 'slf,
    {
        if self.buffer_position() > self.reader.max_len() {
            return Err(unexp_eof());
        }
        let start = self.buffer_position();

        let end = start + bytes;

        if end > self.reader.max_len() {
            return Err(unexp_eof());
        }

        self.state.increment_offset(bytes);

        let slice = self.reader.get_slice(start, end).unwrap();

        Ok(slice)
    }

    /// Returns a statically sized array
    pub(crate) fn read_exact_size<'slf, const SIZE: usize>(
        &'slf mut self,
    ) -> ReadResult<BytesConst<'slc, SIZE>>
    where
        'slc: 'slf,
    {
        let slice = self.read_exact(SIZE)?;

        slice
            .try_into()
            .map_err(|e| inv_data(self, format!("{e:?}")))
    }

    /// Get the next byte without incrementing
    #[allow(dead_code)]
    pub(super) fn peak_next<'slf>(&'slf mut self) -> ReadResult<u8>
    where
        'slc: 'slf,
    {
        let res = self
            .reader
            .get_byte(self.buffer_position())
            .ok_or(unexp_eof())?;
        Ok(res)
    }
    pub(crate) fn read_next<'slf>(&'slf mut self) -> ReadResult<u8>
    where
        'slc: 'slf,
    {
        let res = self
            .reader
            .get_byte(self.buffer_position())
            .ok_or(unexp_eof())?;
        self.state.increment_offset(1);

        Ok(res)
    }
    /// ASSUMING that the offset is pointing at the length of a varlen,
    /// it will read that length and return the resulting slice.
    pub(crate) fn read_varlen_slice<'slf>(&'slf mut self) -> ReadResult<Bytes<'slc>>
    where
        'slc: 'slf,
    {
        let size = decode_varlen(self)?;
        self.read_exact(size as usize)
    }
}

pub(crate) fn decode_varlen<'slc, R: MidiSource<'slc>>(reader: &mut Reader<R>) -> ReadResult<u32> {
    let mut dec: u32 = 0;

    for _ in 0..4 {
        let next = reader.read_next()?;
        dec <<= 7;
        let add = u32::from(next & 0x7F);
        dec |= add;

        //need to continue
        if next & 0x80 != 0x80 {
            break;
        }
    }

    Ok(dec)
}

/// grabs the next byte from the reader and checks it's a u4
#[allow(dead_code)]
pub(crate) fn check_u4(reader: &mut Reader<&[u8]>) -> ReadResult<u8> {
    let byte = reader.read_next()?;
    (byte & 0b1111_0000 == 0)
        .then_some(byte)
        .ok_or(inv_data(reader, "Leading bit found"))
}

impl<'slc, R: MidiSource<'slc>> Reader<R> {
    /// Read the buffer and return an event
    ///
    /// # Errors
    ///
    /// If the next set of bytes are invalid given the current state of the reader
    pub fn read_event<'a>(&mut self) -> ReadResult<FileEvent<'a>>
    where
        'slc: 'a,
    {
        let mut running_status = None;
        let event = loop {
            match self.state.parse_state() {
                ParseState::Init => {
                    self.state.set_parse_state(ParseState::InsideMidi);
                    continue;
                }
                ParseState::InsideMidi => {
                    // expect only a header or track chunk
                    let chunk = match self.read_exact(4) {
                        Ok(c) => c,
                        Err(e) => {
                            if e.is_eof() {
                                return Ok(FileEvent::EOF);
                            } else {
                                return Err(e);
                            }
                        }
                    };

                    match chunk.as_ref() {
                        b"MThd" => {
                            //HeaderChunk should handle us
                            break FileEvent::Header(RawHeaderChunk::read(self)?);
                        }
                        b"MTrk" => {
                            let chunk = TrackChunkHeader::read(self)?;
                            //todo: set new state
                            self.state.set_parse_state(ParseState::InsideTrack {
                                start: self.buffer_position(),
                                length: chunk.len() as usize,
                                prev_status: None,
                            });
                            break FileEvent::Track(chunk);
                        }
                        _ => {
                            //let chunk
                            let chunk = UnknownChunk::read(chunk, self)?;

                            break FileEvent::Unknown(chunk);
                        }
                    }
                }
                ParseState::InsideTrack {
                    start,
                    length,
                    prev_status,
                } => {
                    if start + length <= self.buffer_position() {
                        //end of track events
                        self.state.set_parse_state(ParseState::InsideMidi);
                        continue;
                    }
                    running_status = prev_status;

                    let ev = TrackEvent::read(self, &mut running_status)?;
                    break FileEvent::TrackEvent(ev);
                }
                ParseState::Done => break FileEvent::EOF,
            }
        };

        if let ParseState::InsideTrack { prev_status, .. } = self.state.parse_state_mut() {
            *prev_status = running_status;
        };
        Ok(event)
    }

    /// Read the buffer and return a chunk
    ///
    /// # Errors
    ///
    /// If the next set of bytes are invalid given the current state of the reader
    pub fn read_chunk<'a>(&mut self) -> ReadResult<ChunkEvent<'a>>
    where
        'slc: 'a,
    {
        let event = loop {
            match self.state.parse_state() {
                ParseState::Init => {
                    self.state.set_parse_state(ParseState::InsideMidi);
                    continue;
                }
                ParseState::InsideMidi => {
                    // expect only a header or track chunk
                    let chunk = match self.read_exact(4) {
                        Ok(c) => c,
                        Err(e) => {
                            if e.is_eof() {
                                // Inside Midi + UnexpectedEof should only fire at the end of a file.
                                self.state.set_parse_state(ParseState::Done);
                                return Ok(ChunkEvent::EOF);
                            } else {
                                return Err(e);
                            }
                        }
                    };
                    let chunk_name = chunk.as_ref();

                    match chunk_name {
                        b"MThd" => {
                            //HeaderChunk should handle us
                            break ChunkEvent::Header(RawHeaderChunk::read(self)?);
                        }
                        b"MTrk" => {
                            let chunk = RawTrackChunk::read(self)?;
                            break ChunkEvent::Track(chunk);
                        }
                        _ => {
                            //let chunk
                            let chunk = UnknownChunk::read(chunk, self)?;
                            break ChunkEvent::Unknown(chunk);
                        }
                    };
                }
                ParseState::InsideTrack {
                    start,
                    length,
                    prev_status: _,
                } => {
                    /*
                    If this happens, then read_event was previously called.
                    We will just skip to the end of this track and continue
                    */

                    self.state.set_offset(start + length);
                    self.state.set_parse_state(ParseState::InsideMidi);
                    continue;
                }
                ParseState::Done => break ChunkEvent::EOF,
            }
        };

        Ok(event)
    }
}
