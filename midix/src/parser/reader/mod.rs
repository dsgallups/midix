mod state;
pub use state::*;
mod error;
pub use error::*;
mod event;
pub use event::*;
mod header;
pub use header::*;
mod track;
pub use track::*;

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
}
