mod state;
pub use state::*;
mod error;
pub use error::*;
mod event;
pub use event::*;
mod header_chunk;
pub use header_chunk::*;
mod track_chunk;
pub use track_chunk::*;
mod track_event;
pub use track_event::*;
mod track_message;
pub use track_message::*;

use std::io::{BufRead, BufReader, Read};

use crate::{prelude::MidiChunk, utils::decode_varlen};

#[derive(Clone)]
pub struct Reader<R> {
    reader: R,
    state: ReaderState,
}

impl<R> Reader<R> {
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

    pub const fn buffer_position(&self) -> usize {
        self.state.offset()
    }

    pub const fn increment_buffer_position(&mut self, amt: usize) {
        self.state.increment_offset(amt);
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

impl<R: Read> Reader<BufReader<R>> {
    pub fn from_reader(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
            state: ReaderState::default(),
        }
    }
}

impl<R: BufRead> Reader<R> {
    pub const fn from_buf_reader(reader: R) -> Self {
        Self {
            reader,
            state: ReaderState::default(),
        }
    }
}

impl<'slc> Reader<&'slc [u8]> {
    pub const fn from_byte_slice(slice: &'slc [u8]) -> Self {
        Self {
            reader: slice,
            state: ReaderState::default(),
        }
    }

    pub fn read_event<'a>(&mut self) -> ReadResult<Event<'a>>
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
                    // expect a header or track chunk
                    let chunk = self.read_exact(4)?;
                    match chunk {
                        b"MThd" => {
                            //HeaderChunk should handle us
                            break Event::Header(HeaderChunk::read(self)?);
                        }
                        b"MTrk" => {
                            let chunk = TrackChunk::read(self)?;
                            //todo: set new state
                            self.state.set_parse_state(ParseState::InsideTrack {
                                start: self.buffer_position(),
                                length: chunk.length() as usize,
                            });
                            break Event::Track(chunk);
                        }
                        bytes => {
                            self.state.set_parse_state(ParseState::Done);
                            self.state.set_last_error_offset(self.buffer_position());
                            return Err(inv_data(
                                self.buffer_position(),
                                format!(
                                    "Expected a MIDI Chunk header. Found unexpected input: {:?}",
                                    bytes
                                ),
                            ));
                        }
                    }
                }
                ParseState::InsideTrack { start, length } => {
                    //todo
                    todo!()
                    //todo
                }
                _ => todo!(),
            }
        };
        todo!();

        todo!()
    }
}

//internal implementations
impl<'slc> Reader<&'slc [u8]> {
    // Returns None if there's no bytes left to read
    fn read_exact<'slf>(&'slf mut self, bytes: usize) -> ReadResult<&'slc [u8]>
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
    pub fn read_exact_size<'slf, const SIZE: usize>(&'slf mut self) -> ReadResult<&'slc [u8; SIZE]>
    where
        'slc: 'slf,
    {
        let slice = self.read_exact(SIZE)?;

        Ok(slice
            .try_into()
            .map_err(|e| inv_data(self.buffer_position(), format!("{:?}", e)))?)
    }
}
