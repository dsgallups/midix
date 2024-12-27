use crate::prelude::*;

pub mod chunk;
pub mod format;
pub mod header;
pub mod meta;
pub mod track;

pub struct MidiFile {
    header: MidiHeader,
    tracks: MidiFormat,
}

impl MidiFile {
    pub fn parse(bytes: Vec<u8>) -> ReadResult<Self> {
        todo!();
    }
    pub fn header(&self) -> &MidiHeader {
        &self.header
    }
}

pub struct MidiFileRef<'a> {
    chunks: Vec<MidiChunk<'a>>,
}

impl<'a> MidiFileRef<'a> {
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
    pub fn chunks(&self) -> &[MidiChunk<'a>] {
        &self.chunks
    }
}
