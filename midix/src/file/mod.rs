//pub mod builder;
pub mod chunk;
pub mod format;
pub mod header;
pub mod meta;
pub mod track;

/// Represents a 4 character type
///
/// Each chunk has a 4-character type and a 32-bit length,
/// which is the number of bytes in the chunk. This structure allows
/// future chunk types to be designed which may be easily be ignored
/// if encountered by a program written before the chunk type is introduced.
#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum MidiChunkType {
    /// Represents the byte length of the midi header.
    ///
    /// Begins with "MThd"
    Header,
    /// Represents the byte length of a midi track
    ///
    /// Begins with "MTrk"
    Track,
    /// A chunk type that is not known by this crate
    Unknown,
}
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
