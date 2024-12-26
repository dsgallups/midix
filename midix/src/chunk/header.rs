use crate::chunk::ReaderError;

use super::{ReadResult, Reader};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MidiHeader {
    length: u32,
    format: MidiFormat,
    timing: MidiTiming,
}

impl MidiHeader {
    /// Assumes that the chunk type bytes ("MThd") have ALREADY been read
    pub fn read(reader: &mut Reader<&[u8]>) -> ReadResult<Self> {
        let length = super::read_u32(reader)?;
        let format_b = super::read_u16(reader)?;
        let num_tracks = super::read_u16(reader)?;

        let format = match format_b {
            0 => MidiFormat::SingleMultiChannel, // Always 1 track
            1 => MidiFormat::Simultaneous(num_tracks),
            2 => MidiFormat::SequentiallyIndependent(num_tracks),
            _ => return Err(ReaderError::invalid_input("Invalid MIDI format")),
        };

        let timing = MidiTiming::read(reader)?;

        Ok(Self {
            length,
            format,
            timing,
        })
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn format(&self) -> u8 {
        todo!()
    }
    pub fn num_tracks(&self) -> u16 {
        self.format.num_tracks()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiFormat {
    SingleMultiChannel,
    Simultaneous(u16),
    SequentiallyIndependent(u16),
}

impl MidiFormat {
    pub fn num_tracks(&self) -> u16 {
        use MidiFormat::*;
        match self {
            SingleMultiChannel => 1,
            Simultaneous(num) | SequentiallyIndependent(num) => *num,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiTiming {
    TicksPerQuarterNote(u16),
    NegativeSmtpe { format: Fps, resolution: u8 },
}

impl MidiTiming {
    /// Assumes the next two bytes are for a midi division.
    pub fn read(reader: &mut Reader<&[u8]>) -> ReadResult<Self> {
        let byte = super::read_u16(reader)?;
        match byte >> 15 {
            0 => {
                //this is ticks per quarter_note
                let tpq = byte & 0x0FFF;
                Ok(MidiTiming::TicksPerQuarterNote(tpq))
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
}

/// One of the four FPS values available for SMPTE times, as defined by the MIDI standard.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Fps {
    /// 24 frames per second.
    Fps24,
    /// 25 frames per second.
    Fps25,
    /// Actually `29.97 = 30 / 1.001` frames per second.
    ///
    /// Quite an exotic value because of interesting historical reasons.
    Fps29,
    /// 30 frames per second.
    Fps30,
}
