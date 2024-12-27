use crate::prelude::*;

// I would like to return some type of reader...
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrackChunk<'a> {
    length: u32,
    data: &'a [u8],
}

impl<'a> TrackChunk<'a> {
    /// Assumes that the chunk type bytes ("MTrk") have ALREADY been read
    pub fn read<'r, 'slc>(reader: &'r mut OldReader<&'slc [u8]>) -> ReadResult<Self>
    where
        'slc: 'a,
    {
        let length: &[u8; 4] = reader.read_exact_size()?;

        let length = u32::from_be_bytes(*length);

        let track_event_bytes = reader.read_exact(length as usize)?;

        Ok(Self {
            length,
            data: track_event_bytes,
        })
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    /// Slow, can be improved by implementing iterator on reader
    pub fn events(&self) -> ReadResult<Vec<MidiTrackEventRef<'a>>> {
        let mut reader = OldReader::from_byte_slice(self.data);

        let mut events: Vec<MidiTrackEventRef<'a>> = Vec::new();
        loop {
            match MidiTrackEventRef::read(&mut reader) {
                Ok(e) => events.push(e),
                Err(err) => match err {
                    ReaderError::EndOfReader => break,
                    e => return Err(e),
                },
            }
        }

        Ok(events)
    }
}

#[test]
fn test_simple_sysex() {
    let bytes = [0xF0, 0x05, 0x43, 0x12, 0x00, 0x07, 0xF7];
    let mut reader = OldReader::from_byte_slice(&bytes);
    let msg = MidiTrackMessageRef::read(&mut reader).unwrap();

    assert_eq!(
        msg,
        MidiTrackMessageRef::SystemExclusive(SystemExclusiveRef::new(&[0x43, 0x12, 0x00, 0x07]))
    );
}
