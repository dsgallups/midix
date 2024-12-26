use crate::prelude::*;
#[cfg(test)]
use pretty_assertions::assert_eq;

/// Represents a sequence of bytes that identify the length of a chunk
/// and its type.
///
/// Each chunk has a 4-character type and a 32-bit length,
/// which is the number of bytes in the chunk. This structure allows
/// future chunk types to be designed which may be easily be ignored
/// if encountered by a program written before the chunk type is introduced.
#[derive(Debug, Clone, PartialEq)]
pub enum MidiChunkType {
    /// Represents the byte length of the midi header
    Header(u32),
    /// Represents the byte length of a midi track
    Track(u32),
    /// A chunk type that is not known by this crate
    Unknown(u32),
}

impl MidiChunkType {
    pub fn read(reader: &mut Reader<&[u8]>) -> ReadResult<Self> {
        let initial_offset = reader.buffer_position();
        let chunk_type = reader.read_exact(4)?;
        let chunk_size: &[u8; 4] = reader.read_exact_size()?;
        // this takes some time but like, it's pretty fast
        let chunk_size = u32::from_be_bytes(*chunk_size);
        match chunk_type {
            b"MThd" => Ok(Self::Header(chunk_size)),
            b"MTrk" => Ok(Self::Track(chunk_size)),
            _ => Err(ReaderError::invalid_input(format!(
                "Invalid chunk header at {}",
                initial_offset
            ))),
        }
    }
}

#[test]
fn test_reader_header_chunk() {
    let bytes = [0x4D, 0x54, 0x68, 0x64, 0x00, 0x00, 0x00, 0x06];
    let mut reader = Reader::from_byte_slice(&bytes);

    let result = MidiChunkType::read(&mut reader).unwrap();

    assert_eq!(result, MidiChunkType::Header(6));
}

#[test]
fn test_reader_track_chunk() {
    let bytes = [0x4D, 0x54, 0x72, 0x6B, 0x00, 0x00, 0x05, 0x29];
    let mut reader = Reader::from_byte_slice(&bytes);

    let result = MidiChunkType::read(&mut reader).unwrap();

    assert_eq!(result, MidiChunkType::Track(1321));
}
