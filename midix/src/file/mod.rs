#![doc = r#"
Rusty representation of a [`MidiFile`]

TODO
"#]

mod builder;
use std::borrow::Cow;

use builder::*;
mod format;
pub use format::*;
mod header;
pub use header::*;
mod track;
pub use track::*;

use crate::reader::{ReadResult, Reader};

#[doc = r#"
TODO
"#]
pub struct MidiFile<'a> {
    header: Header<'a>,
    format: Format<'a>,
}

impl<'a> MidiFile<'a> {
    /// Parse a set of bytes into a file struct
    pub fn parse<B>(bytes: B) -> ReadResult<Self>
    where
        B: Into<Cow<'a, [u8]>>,
    {
        let mut reader = Reader::from_bytes(bytes);
        let mut builder = MidiFileBuilder::default();

        loop {
            let val = reader.read_chunk().unwrap();

            if val.is_eof() {
                break;
            }
            builder.handle_chunk(val)?;
        }

        builder.build()
    }

    /// Returns header info
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Returns a track list
    pub fn tracks(&self) -> Vec<&Track<'a>> {
        match self.format {
            Format::SequentiallyIndependent(ref t) => t.iter().collect(),
            Format::Simultaneous(ref s) => s.iter().collect(),
            Format::SingleMultiChannel(ref c) => vec![c],
        }
    }
}
