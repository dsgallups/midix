pub mod header;

use crate::prelude::*;
use header::MidiHeader;
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
pub enum MidiChunkHeader {
    /// Represents the byte length of the midi header
    ///
    /// Begins with "MThd"
    Header(MidiHeader),
    /// Represents the byte length of a midi track
    ///
    /// Begins with "MTrk"
    Track { length: u32 },
    /// A chunk type that is not known by this crate
    Unknown { length: u32 },
}

/// Represents a 4 character type
///
/// Each chunk has a 4-character type and a 32-bit length,
/// which is the number of bytes in the chunk. This structure allows
/// future chunk types to be designed which may be easily be ignored
/// if encountered by a program written before the chunk type is introduced.
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl MidiChunkHeader {
    pub fn read(reader: &mut Reader<&[u8]>) -> ReadResult<Self> {
        let chunk_type = reader.read_exact(4)?;
        let chunk_size: &[u8; 4] = reader.read_exact_size()?;
        // this takes some time but like, it's pretty fast
        let length = u32::from_be_bytes(*chunk_size);
        Ok(match chunk_type {
            b"MThd" => Self::Header(MidiHeader::read(reader)?),
            b"MTrk" => Self::Track { length },
            _ => Self::Unknown { length },
        })
    }

    pub fn chunk_type(&self) -> MidiChunkType {
        use MidiChunkHeader::*;
        match self {
            Header { .. } => MidiChunkType::Header,
            Track { .. } => MidiChunkType::Track,
            Unknown { .. } => MidiChunkType::Unknown,
        }
    }

    pub fn length(&self) -> u32 {
        todo!();
    }
}

#[test]
fn test_reader_header_chunk() {
    let bytes = [0x4D, 0x54, 0x68, 0x64, 0x00, 0x00, 0x00, 0x06];
    let mut reader = Reader::from_byte_slice(&bytes);

    let result = MidiChunkHeader::read(&mut reader).unwrap();

    assert_eq!(result.chunk_type(), MidiChunkType::Header);
    assert_eq!(result.length(), 6);
}

#[test]
fn test_reader_track_chunk() {
    let bytes = [0x4D, 0x54, 0x72, 0x6B, 0x00, 0x00, 0x05, 0x29];
    let mut reader = Reader::from_byte_slice(&bytes);

    let result = MidiChunkHeader::read(&mut reader).unwrap();

    assert_eq!(result.chunk_type(), MidiChunkType::Track);
    assert_eq!(result.length(), 1321);
}

fn read_u32(reader: &mut Reader<&[u8]>) -> ReadResult<u32> {
    let chunk_size: &[u8; 4] = reader.read_exact_size()?;
    // this takes some time but like, it's pretty fast
    Ok(u32::from_be_bytes(*chunk_size))
}
fn read_u16(reader: &mut Reader<&[u8]>) -> ReadResult<u16> {
    let chunk_size: &[u8; 2] = reader.read_exact_size()?;
    // this takes some time but like, it's pretty fast
    Ok(u16::from_be_bytes(*chunk_size))
}
