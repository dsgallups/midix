#![doc = r#"
# Reader for parsing midi

Inspired by <https://docs.rs/quick-xml/latest/quick_xml/>


## TODO
- [ ] Config
"#]

use error::{ReadResult, ReaderError};
use state::ReaderState;

pub mod error;
pub(crate) mod state;

#[derive(Clone)]
pub struct Reader<R> {
    reader: R,
    state: ReaderState,
}

impl<R> Reader<R> {
    pub const fn from_reader(reader: R) -> Self {
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

impl<'slc> Reader<&'slc [u8]> {
    pub const fn from_byte_slice(slice: &'slc [u8]) -> Self {
        Self {
            reader: slice,
            state: ReaderState::default(),
        }
    }

    pub fn read_next<'slf>(&'slf mut self) -> ReadResult<&'slc u8>
    where
        'slc: 'slf,
    {
        let res = self
            .reader
            .get(self.buffer_position())
            .ok_or(ReaderError::end())?;
        self.state.increment_offset(1);

        Ok(res)
    }

    /// ASSUMING that the offset is pointing at the length of a varlen,
    /// it will read that length and return the resulting slice.
    pub(crate) fn read_varlen<'slf>(&'slf mut self) -> ReadResult<&'slc [u8]>
    where
        'slc: 'slf,
    {
        let size = *self.read_next()?;
        self.read_exact(size as usize)
    }

    /// Peak the next set of bytes without incrementing the buffer position.
    ///
    /// Errors if the peak goes out of bounds from the slice.
    ///
    /// Will not increment the last error offset.
    pub fn peek_exact<'slf>(&'slf mut self, bytes: usize) -> ReadResult<&'slc [u8]>
    where
        'slc: 'slf,
    {
        let start = self.buffer_position();
        let end = start + bytes;

        if end > self.reader.len() {
            return Err(ReaderError::end());
        }

        let slice = &self.reader[start..end];

        Ok(slice)
    }

    pub fn read_exact<'slf>(&'slf mut self, bytes: usize) -> ReadResult<&'slc [u8]>
    where
        'slc: 'slf,
    {
        let start = self.buffer_position();
        let end = start + bytes;

        if end > self.reader.len() {
            self.state.increment_last_error_offset(self.reader.len());
            return Err(ReaderError::end());
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
        let start = self.buffer_position();
        let end = start + SIZE;
        self.state.increment_offset(SIZE);

        if end > self.reader.len() {
            println!(
                "Error: start: {}, end: {}, len: {}",
                start,
                end,
                self.reader.len()
            );
            self.state.increment_last_error_offset(self.reader.len());
            return Err(ReaderError::end());
        }

        let slice = &self.reader[start..end];

        slice
            .try_into()
            .map_err(|_| ReaderError::invalid_input("Invalid length"))
    }
}

#[test]
fn test_read_bytes() {
    let bytes = [
        0x00, 0x00, 0x00, 0x06, //length
        0x00, 0x01, //format
        0x00, 0x03, //num_tracks
        0x00, 0x78, //timing
    ];
    let mut reader = Reader::from_byte_slice(&bytes);

    reader.read_exact(4).unwrap();
    reader.read_exact(2).unwrap();
    reader.read_exact(2).unwrap();
    reader.read_exact(2).unwrap();

    assert_eq!(reader.buffer_position(), 10);
}
