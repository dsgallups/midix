use crate::prelude::*;

/// Represents a sequence of bytes that identify the length of a chunk
/// and its type.
///
/// Each chunk has a 4-character type and a 32-bit length,
/// which is the number of bytes in the chunk. This structure allows
/// future chunk types to be designed which may be easily be ignored
/// if encountered by a program written before the chunk type is introduced.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MidiChunk<'a> {
    /// Represents the byte length of the midi header
    ///
    /// Begins with "MThd"
    Header(MidiHeaderRef<'a>),
    /// Represents the byte length of a midi track
    ///
    /// Begins with "MTrk"
    Track(MidiTrackRef<'a>),
    /// A chunk type that is not known by this crate
    Unknown { length: &'a [u8; 4], data: &'a [u8] },
}

impl<'a> MidiChunk<'a> {
    pub fn read<'r, 'slc>(reader: &'r mut OldReader<&'slc [u8]>) -> OldReadResult<Self>
    where
        'slc: 'a,
    {
        let chunk_type = reader.read_exact(4)?;
        Ok(match chunk_type {
            b"MThd" => Self::Header(MidiHeaderRef::read(reader)?),
            b"MTrk" => Self::Track(MidiTrackRef::read(reader)?),
            _ => {
                let length: &[u8; 4] = reader.read_exact_size()?;
                let chunk_size = u32::from_be_bytes(*length);
                //increment the reader offset by the size
                let data = reader.read_exact(chunk_size as usize).unwrap_or_default();

                Self::Unknown { length, data }
            }
        })
    }

    pub fn chunk_type(&self) -> MidiChunkType {
        use MidiChunk::*;
        match self {
            Header { .. } => MidiChunkType::Header,
            Track { .. } => MidiChunkType::Track,
            Unknown { .. } => MidiChunkType::Unknown,
        }
    }

    pub fn length(&self) -> u32 {
        use MidiChunk::*;
        match self {
            Header(h) => h.length(),
            Track(t) => t.length(),
            Unknown { length, .. } => convert_u32(length),
        }
    }
}

#[test]
fn test_reader_header_chunk() {
    let bytes = [
        0x4D, 0x54, 0x68, 0x64, //header MThd
        0x00, 0x00, 0x00, 0x06, //length
        0x00, 0x01, //format
        0x00, 0x03, //num_tracks
        0x00, 0x78, //timing
    ];
    let mut reader = OldReader::from_byte_slice(&bytes);

    let result = MidiChunk::read(&mut reader).unwrap();

    assert_eq!(result.chunk_type(), MidiChunkType::Header);
    assert_eq!(result.length(), 6);
}

#[test]
fn test_unknown_chunk() {
    let bytes = [
        0x4D, 0x54, 0x72, 0x6C, //header MThe (unknown)
        0x00, 0x00, 0x00, 0x08, // length
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // length of unknown
        0x4D, 0x54, 0x68, 0x64, //header MThd
        0x00, 0x00, 0x00, 0x06, //length
        0x00, 0x01, //format
        0x00, 0x03, //num_tracks
        0x00, 0x78, //timing
    ];
    let mut reader = OldReader::from_byte_slice(&bytes);

    let result = MidiChunk::read(&mut reader).unwrap();

    assert_eq!(result.chunk_type(), MidiChunkType::Unknown);
    assert_eq!(result.length(), 8);

    let header = MidiChunk::read(&mut reader).unwrap();
    assert_eq!(header.chunk_type(), MidiChunkType::Header);
    assert_eq!(header.length(), 6);
}
