use super::{ReadResult, Reader};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MidiTrack {
    length: u32,
}

impl MidiTrack {
    pub fn read(reader: &mut Reader<&[u8]>) -> ReadResult<Self> {
        let length = super::read_u32(reader)?;
        Ok(Self { length })
    }
    pub fn length(&self) -> u32 {
        self.length
    }
}
