use std::borrow::Cow;

use crate::prelude::*;

/// The header timing type.
///
/// This is either the number of ticks per quarter note or
/// the alternative SMTPE format. See the [`HeaderChunk`] docs for more information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Timing<'a> {
    /// The midi file's delta times are defined using a tick rate per quarter note
    TicksPerQuarterNote(Cow<'a, [u8; 2]>),

    /// The midi file's delta times are defined using an SMPTE and MIDI Time Code
    NegativeSmpte(Cow<'a, [u8; 2]>),
}

impl<'a> Timing<'a> {
    /// Create a new timing from a 2 byte array slice.
    pub fn new_ticks_from_slice(arr: &'a [u8; 2]) -> Self {
        Self::TicksPerQuarterNote(Cow::Borrowed(arr))
    }

    /// Create a new negative SMPTE division a 2 byte array slice
    pub fn new_negative_smpte_from_slice(arr: &'a [u8; 2]) -> Self {
        Self::NegativeSmpte(Cow::Borrowed(arr))
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
