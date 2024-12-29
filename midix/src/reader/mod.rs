#![doc = r"
Contains a high-level interface for a pull-based MIDI file parser

See the [`Reader`] docs for more information

# Acknowledgments

Inspired by <https://docs.rs/quick-xml/latest/quick_xml/>
"]

mod error;
mod state;
pub use error::*;
use state::{ParseState, ReaderState};

use std::{borrow::Cow, io::ErrorKind};

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

# Examples
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
    Timing::new_ticks(&[0, 96]).ticks_per_quarter_note()
);

"#]
#[derive(Clone)]
pub struct Reader<R> {
    reader: R,
    state: ReaderState,
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

    /// Read the buffer and return an event
    ///
    /// # Errors
    ///
    /// If the next set of bytes are invalid given the current state of the reader
    pub fn read_event<'a>(&mut self) -> ReadResult<FileEvent<'a>>
    where
        'slc: 'a,
    {
        let event = loop {
            println!("state: {:?}", self.state.parse_state());
            match self.state.parse_state() {
                ParseState::Init => {
                    self.state.set_parse_state(ParseState::InsideMidi);
                    continue;
                }
                ParseState::InsideMidi => {
                    // expect only a header or track chunk
                    let chunk = match self.read_exact(4) {
                        Ok(c) => c,
                        Err(e) => match e.kind() {
                            // Inside Midi + UnexpectedEof should only fire at the end of a file.
                            ErrorKind::UnexpectedEof => {
                                self.state.set_parse_state(ParseState::Done);
                                return Ok(FileEvent::eof());
                            }
                            _ => {
                                return Err(e);
                            }
                        },
                    };
                    match chunk {
                        b"MThd" => {
                            //HeaderChunk should handle us
                            break FileEvent::Header(HeaderChunk::read(self)?);
                        }
                        b"MTrk" => {
                            let chunk = TrackChunk::read(self)?;
                            //todo: set new state
                            self.state.set_parse_state(ParseState::InsideTrack {
                                start: self.buffer_position(),
                                length: chunk.len() as usize,
                                prev_status: None,
                            });
                            break FileEvent::Track(chunk);
                        }
                        bytes => {
                            //let chunk
                            let chunk = UnknownChunk::read(bytes, self)?;

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
                    //need to do this procedurally due to running statuses on midi events
                    let delta_time = decode_varlen(self)?;

                    let next_event = self.read_next()?;

                    let message = match next_event {
                        0xF0 => {
                            let mut data = self.read_varlen_slice()?;
                            if !data.is_empty() {
                                //discard the last 0xF7
                                data = &data[..data.len() - 1];
                            }
                            TrackMessage::SystemExclusive(SysEx::new_borrowed(data))
                        }
                        0xFF => TrackMessage::Meta(Meta::read(self)?),
                        byte => {
                            //status if the byte has a leading 1, otherwise it's
                            //a running status
                            let status = if byte >> 7 == 1 {
                                let ParseState::InsideTrack {
                                    ref mut prev_status,
                                    ..
                                } = self.state.parse_state_mut()
                                else {
                                    return Err(inv_data(
                                        self,
                                        "Encountered midi event outside of track",
                                    ));
                                };
                                *prev_status = Some(*byte);

                                Cow::Borrowed(byte)
                            } else if let Some(prev_status) = prev_status {
                                Cow::Owned(prev_status)
                            } else {
                                return Err(inv_data(self, "Invalid MIDI event triggered"));
                            };

                            //todo
                            TrackMessage::ChannelVoice(ChannelVoice::read(status, self)?)
                        }
                    };

                    break FileEvent::TrackEvent(TrackEvent::new(delta_time, message));
                }
                ParseState::Done => break FileEvent::EOF,
            }
        };
        Ok(event)
    }
}

//internal implementations
impl<'slc> Reader<&'slc [u8]> {
    // Returns None if there's no bytes left to read
    pub(super) fn read_exact<'slf>(&'slf mut self, bytes: usize) -> ReadResult<&'slc [u8]>
    where
        'slc: 'slf,
    {
        if self.buffer_position() >= self.reader.len() {
            return Err(unexp_eof());
        }
        let start = self.buffer_position();

        let end = start + bytes;

        if end > self.reader.len() {
            return Err(unexp_eof());
        }

        self.state.increment_offset(bytes);

        let slice = &self.reader[start..end];

        Ok(slice)
    }
    /// Returns a statically sized array
    pub(crate) fn read_exact_size<'slf, const SIZE: usize>(
        &'slf mut self,
    ) -> ReadResult<&'slc [u8; SIZE]>
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
    pub(super) fn peak_next<'slf>(&'slf mut self) -> ReadResult<&'slc u8>
    where
        'slc: 'slf,
    {
        let res = self.reader.get(self.buffer_position()).ok_or(unexp_eof())?;
        Ok(res)
    }
    pub(crate) fn read_next<'slf>(&'slf mut self) -> ReadResult<&'slc u8>
    where
        'slc: 'slf,
    {
        let res = self.reader.get(self.buffer_position()).ok_or(unexp_eof())?;
        self.state.increment_offset(1);

        Ok(res)
    }
    /// ASSUMING that the offset is pointing at the length of a varlen,
    /// it will read that length and return the resulting slice.
    pub(crate) fn read_varlen_slice<'slf>(&'slf mut self) -> ReadResult<&'slc [u8]>
    where
        'slc: 'slf,
    {
        let size = decode_varlen(self)?;
        self.read_exact(size as usize)
    }
}

pub(super) fn decode_varlen(reader: &mut Reader<&[u8]>) -> ReadResult<u32> {
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

/// grabs the next byte from the reader and checks it's a u7
pub(crate) fn check_u7<'slc>(reader: &mut Reader<&'slc [u8]>) -> ReadResult<&'slc u8> {
    let byte = reader.read_next()?;
    (byte & 0b1000_0000 == 0)
        .then_some(byte)
        .ok_or(inv_data(reader, "Leading bit found"))
}

/// grabs the next byte from the reader and checks it's a u4
#[allow(dead_code)]
pub(crate) fn check_u4<'slc>(reader: &mut Reader<&'slc [u8]>) -> ReadResult<&'slc u8> {
    let byte = reader.read_next()?;
    (byte & 0b1111_0000 == 0)
        .then_some(byte)
        .ok_or(inv_data(reader, "Leading bit found"))
}
