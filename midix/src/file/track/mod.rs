mod event;
pub use event::*;
mod message;
pub use message::*;

use crate::prelude::*;

// I would like to return some type of reader...
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrackChunk {
    length: u32,
}

impl TrackChunk {
    /// Assumes that the chunk type bytes ("MTrk") have ALREADY been read
    pub fn read(reader: &mut Reader<&[u8]>) -> ReadResult<Self> {
        let length: &[u8; 4] = reader.read_exact_size()?;

        let length = u32::from_be_bytes(*length);

        Ok(Self { length })
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    /*/// Slow, can be improved by implementing iterator on reader
    pub fn events(&self) -> OldReadResult<Vec<MidiTrackEventRef<'a>>> {
        let mut reader = OldReader::from_byte_slice(self.data);

        let mut events: Vec<MidiTrackEventRef<'a>> = Vec::new();
        loop {
            match MidiTrackEventRef::read(&mut reader) {
                Ok(e) => events.push(e),
                Err(err) => match err {
                    OldReaderError::EndOfReader => break,
                    e => return Err(e),
                },
            }
        }

        Ok(events)
    }*/
}
