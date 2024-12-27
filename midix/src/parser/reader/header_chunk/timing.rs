use reader::{ReadResult, Reader};

use crate::prelude::*;

use super::inv_data;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Timing<'a> {
    TicksPerQuarterNote(&'a [u8; 2]),
    NegativeSmpte(&'a [u8; 2]),
}

impl<'a> Timing<'a> {
    /// Assumes the next two bytes are for a midi division.
    pub fn read<'r, 'slc>(reader: &'r mut Reader<&'slc [u8]>) -> ReadResult<Self>
    where
        'slc: 'a,
    {
        let bytes: &[u8; 2] = reader.read_exact_size()?;
        match bytes[0] >> 7 {
            0 => {
                //this is ticks per quarter_note
                Ok(Timing::TicksPerQuarterNote(bytes))
            }
            1 => Ok(Timing::NegativeSmpte(bytes)),
            t => Err(inv_data(reader, format!("Invalid MIDI Timing type {}", t))),
        }
    }
    /// Returns Some if the midi timing is a tick per quarter note
    pub fn ticks_per_quarter_note(self) -> Option<u16> {
        match self {
            Self::TicksPerQuarterNote(t) => {
                let v = u16::from_be_bytes(*t);
                Some(v & 0x7FFF)
            }
            _ => todo!(),
        }
    }
    pub fn to_owned(self) -> MidiTiming {
        match self {
            Self::TicksPerQuarterNote(t) => {
                let v = u16::from_be_bytes(*t);
                MidiTiming::TicksPerQuarterNote(v & 0x7FFF)
            }
            _ => todo!(),
        }
    }
}
