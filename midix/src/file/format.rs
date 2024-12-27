use super::MidiTrack;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MidiFormat {
    SingleMultiChannel(MidiTrack),
    Simultaneous(Vec<MidiTrack>),
    SequentiallyIndependent(Vec<MidiTrack>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiFormatRef<'a> {
    SingleMultiChannel,
    Simultaneous(&'a [u8; 2]),
    SequentiallyIndependent(&'a [u8; 2]),
}

impl MidiFormatRef<'_> {
    pub fn num_tracks(self) -> u16 {
        use MidiFormatRef::*;
        match self {
            SingleMultiChannel => 1,
            Simultaneous(num) | SequentiallyIndependent(num) => u16::from_be_bytes(*num),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiFormatType {
    SingleMultiChannel,
    Simultaneous,
    SequentiallyIndependent,
}
