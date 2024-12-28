use std::borrow::Cow;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Timing<'a> {
    TicksPerQuarterNote(Cow<'a, [u8; 2]>),
    NegativeSmpte(Cow<'a, [u8; 2]>),
}

impl<'a> Timing<'a> {
    pub fn new_ticks(arr: &'a [u8; 2]) -> Self {
        Self::TicksPerQuarterNote(Cow::Borrowed(arr))
    }

    /// Assumes the next two bytes are for a midi division.
    pub fn read<'r, 'slc>(reader: &'r mut Reader<&'slc [u8]>) -> ReadResult<Self>
    where
        'slc: 'a,
    {
        let bytes: &[u8; 2] = reader.read_exact_size()?;
        match bytes[0] >> 7 {
            0 => {
                //this is ticks per quarter_note
                Ok(Timing::TicksPerQuarterNote(Cow::Borrowed(bytes)))
            }
            1 => Ok(Timing::NegativeSmpte(Cow::Borrowed(bytes))),
            t => Err(inv_data(reader, format!("Invalid MIDI Timing type {}", t))),
        }
    }
    /// Returns Some if the midi timing is a tick per quarter note
    pub fn ticks_per_quarter_note(&self) -> Option<u16> {
        match self {
            Self::TicksPerQuarterNote(t) => {
                let v = u16::from_be_bytes(**t);
                Some(v & 0x7FFF)
            }
            _ => todo!(),
        }
    }
}
