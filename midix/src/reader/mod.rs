#![doc = r#"
# Reader for parsing midi

Inspired by <https://docs.rs/quick-xml/latest/quick_xml/>


## TODO
- [ ] Config
"#]

use state::ReaderState;

pub mod state;

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

    pub const fn buffer_position(&self) -> u64 {
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

impl<'a> Reader<&'a [u8]> {
    pub const fn from_byte_slice(slice: &'a [u8]) -> Self {
        Self {
            reader: slice,
            state: ReaderState::default(),
        }
    }

    pub fn read_event(&self) -> u8 {
        todo!();
    }
}
