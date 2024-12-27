mod state;
pub use state::*;
mod error;
pub use error::*;

use std::io::{BufRead, BufReader, Read};

use crate::prelude::*;

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

    pub fn set_last_error_offset(&mut self, offset: usize) {
        self.state.set_last_error_offset(offset);
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
            println!("state: {:?}", self.state.parse_state());
            match self.state.parse_state() {
                ParseState::Init => {
                    self.state.set_parse_state(ParseState::InsideMidi);
                    continue;
                }
                ParseState::InsideMidi => {
                    // expect only a header or track chunk
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
                                prev_status: None,
                            });
                            break Event::Track(chunk);
                        }
                        bytes => {
                            self.state.set_parse_state(ParseState::Done);
                            return Err(inv_data(
                                self,
                                format!(
                                    "Expected a MIDI Chunk header. Found unexpected input: {:?}",
                                    bytes
                                ),
                            ));
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
                            TrackMessage::SystemExclusive(SysEx::new(data))
                        }
                        0xFF => TrackMessage::Meta(MetaRef::read(self)?),
                        byte => {
                            //status if the byte has a leading 1, otherwise it's
                            //a running status
                            let status = if byte >> 7 == 1 {
                                let ParseState::InsideTrack { prev_status, .. } =
                                    self.state.parse_state_mut()
                                else {
                                    return Err(inv_data(
                                        self,
                                        "Encountered midi event outside of track",
                                    ));
                                };
                                *prev_status = Some(*byte);

                                *byte
                            } else if let Some(prev_status) = prev_status {
                                prev_status
                            } else {
                                return Err(inv_data(self, "Invalid MIDI event triggered"));
                            };

                            //todo
                            TrackMessage::ChannelVoice(ChannelVoiceRef::read(status, self)?)
                        }
                    };

                    break Event::TrackEvent(TrackEvent::new(delta_time, message));
                }
                ParseState::Done => break Event::EOF,
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
            .map_err(|e| inv_data(self, format!("{:?}", e)))
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
        let add = (next & 0x7F) as u32;
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
    (byte & 0b10000000 == 0)
        .then_some(byte)
        .ok_or(inv_data(reader, "Leading bit found"))
}

/// grabs the next byte from the reader and checks it's a u4
#[allow(dead_code)]
pub(crate) fn check_u4<'slc>(reader: &mut Reader<&'slc [u8]>) -> ReadResult<&'slc u8> {
    let byte = reader.read_next()?;
    (byte & 0b11110000 == 0)
        .then_some(byte)
        .ok_or(inv_data(reader, "Leading bit found"))
}
