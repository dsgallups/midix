#![doc = r#"
Rusty representation of a [`MidiFile`]

TODO
"#]

mod builder;

use alloc::{borrow::Cow, vec::Vec};
use builder::*;
mod format;
pub use format::*;
mod header;
pub use header::*;
mod track;
pub use track::*;

use crate::{
    ParseError,
    prelude::FormatType,
    reader::{ReadResult, Reader, ReaderError, ReaderErrorKind},
};

#[doc = r#"
TODO
"#]
pub struct MidiFile<'a> {
    header: Header,
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
            builder
                .handle_chunk(val)
                .map_err(|k| ReaderError::new(reader.buffer_position(), k))?;
        }

        builder.build().map_err(|k| {
            ReaderError::new(
                reader.buffer_position(),
                ReaderErrorKind::ParseError(ParseError::File(k)),
            )
        })
    }
    /// Returns the bpm for the song
    pub fn bpm(&self) -> f64 {
        todo!();
    }

    /// Returns header info
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Returns a track list
    pub fn tracks(&self) -> Vec<&Track<'a>> {
        match &self.format {
            Format::SequentiallyIndependent(t) => t.iter().collect(),
            Format::Simultaneous(s) => s.iter().collect(),
            Format::SingleMultiChannel(c) => [c].to_vec(),
        }
    }
    /// Returns the format type for the file.
    pub fn format_type(&self) -> FormatType {
        match &self.format {
            Format::SequentiallyIndependent(_) => FormatType::SequentiallyIndependent,
            Format::Simultaneous(_) => FormatType::Simultaneous,
            Format::SingleMultiChannel(_) => FormatType::SingleMultiChannel,
        }
    }
}
