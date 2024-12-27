use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiTiming {
    TicksPerQuarterNote(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiTimingRef<'a> {
    TicksPerQuarterNote(&'a [u8; 2]),
}

impl<'a> MidiTimingRef<'a> {
    /// Assumes the next two bytes are for a midi division.
    pub fn read<'r, 'slc>(reader: &'r mut Reader<&'slc [u8]>) -> ReadResult<Self>
    where
        'slc: 'a,
    {
        let bytes: &[u8; 2] = reader.read_exact_size()?;
        match bytes[0] >> 7 {
            0 => {
                //this is ticks per quarter_note
                Ok(MidiTimingRef::TicksPerQuarterNote(bytes))
            }
            1 => {
                //negative smtpe
                Err(ReaderError::unimplemented(
                    "Reading Negative SMPTE midi files is not yet supported",
                ))
            }
            _ => Err(ReaderError::invalid_data()),
        }
    }
    /// Returns Some if the midi timing is a tick per quarter note
    pub fn ticks_per_quarter_note(self) -> Option<u16> {
        match self {
            Self::TicksPerQuarterNote(t) => {
                let v = u16::from_be_bytes(*t);
                Some(v & 0x7FFF)
            }
        }
    }
}
