use crate::prelude::*;

pub mod chunk;
pub mod header;
pub mod meta;
pub mod track;

pub struct MidiFile<'a> {
    chunks: Vec<MidiChunk<'a>>,
}

impl<'a> MidiFile<'a> {
    pub fn read<'r, 'slc>(reader: &'r mut Reader<&'slc [u8]>) -> ReadResult<Self>
    where
        'slc: 'a,
    {
        let mut chunks = Vec::new();

        loop {
            match MidiChunk::read(reader) {
                Ok(chunk) => chunks.push(chunk),
                Err(e) => match e {
                    ReaderError::EndOfReader => break,
                    e => {
                        return Err(e);
                    }
                },
            }
        }

        Ok(Self { chunks })
    }
    pub fn new_read<'i, I>(iterator: I) -> ReadResult<Self>
    where
        I: Iterator<Item = &'i u8>,
        'i: 'a,
    {
        let mut chunks = Vec::new();

        loop {
            match MidiChunk::read(reader) {
                Ok(chunk) => chunks.push(chunk),
                Err(e) => match e {
                    ReaderError::EndOfReader => break,
                    e => {
                        return Err(e);
                    }
                },
            }
        }
        todo!();
    }
    pub fn chunks(&self) -> &[MidiChunk<'a>] {
        &self.chunks
    }
}
