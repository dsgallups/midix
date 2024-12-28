//pub mod builder;
pub mod chunk;
pub mod format;
pub mod header;
pub mod meta;
pub mod track;
/*
pub struct MidiFile {
    header: MidiHeader,
    tracks: FormatOwned,
}

impl MidiFile {
    /*
    pub fn parse(bytes: &[u8]) -> OldReadResult<Self> {
        let mut reader = OldReader::from_byte_slice(bytes);
        let mut builder = MidiFileBuilder::default();
        loop {
            match reader.read_chunk() {
                Ok(c) => builder.handle_chunk(c)?,
                Err(e) => match e {
                    OldReaderError::EndOfReader => {
                        break;
                    }
                    e => return Err(e),
                },
            }
        }
        builder.build()
    }*/
    pub fn header(&self) -> &MidiHeader {
        &self.header
    }
    pub fn tracks(&self) -> Vec<&MidiTrack> {
        match self.tracks {
            Format::SequentiallyIndependent(ref t) => t.iter().collect(),
            Format::Simultaneous(ref s) => s.iter().collect(),
            Format::SingleMultiChannel(ref c) => vec![c],
        }
    }
}*/

/*
pub struct MidiFileRef<'a> {
    chunks: Vec<MidiChunk<'a>>,
}

impl<'a> MidiFileRef<'a> {
    pub fn read<'r, 'slc>(reader: &'r mut OldReader<&'slc [u8]>) -> OldReadResult<Self>
    where
        'slc: 'a,
    {
        let mut chunks = Vec::new();

        loop {
            match MidiChunk::read(reader) {
                Ok(chunk) => chunks.push(chunk),
                Err(e) => match e {
                    OldReaderError::EndOfReader => break,
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
*/
