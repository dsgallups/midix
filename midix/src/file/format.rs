use std::borrow::Cow;
/*
use super::MidiTrack;

/*


FF 00 02 Sequence Number
This optional event, which must occur at the beginning of a track,
before any nonzero delta-times, and before any transmittable MIDI
events, specifies the number of a sequence. In a format 2 MIDI File,
it is used to identify each "pattern" so that a "song" sequence using
the Cue message can refer to the patterns. If the ID numbers are
omitted, the sequences' locations in order in the file are used as
defaults. In a format 0 or 1 MIDI File, which only contain one
sequence, this number should be contained in the first (or only)
track. If transfer of several multitrack sequences is required,
this must be done as a group of format 1 files, each with a different
sequence number.
*/

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatOwned {
    /// Format 0
    SingleMultiChannel(MidiTrack),
    /// Format 1
    Simultaneous(Vec<MidiTrack>),
    /// Format 2
    SequentiallyIndependent(Vec<MidiTrack>),
}*/

/*#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SequenceTrack {
    track: MidiTrack,
    sequence_number: u16,
}
impl SequenceTrack {
    pub fn new(track: MidiTrack, sequence_number: u16) -> Self {
        Self {
            track,
            sequence_number,
        }
    }
}*/

#[doc = r#"
    FF 00 02 Sequence Number
    This optional event, which must occur at the beginning of a track,
    before any nonzero delta-times, and before any transmittable MIDI
    events, specifies the number of a sequence. In a format 2 MIDI File,
    it is used to identify each "pattern" so that a "song" sequence using
    the Cue message can refer to the patterns. If the ID numbers are
    omitted, the sequences' locations in order in the file are used as
    defaults. In a format 0 or 1 MIDI File, which only contain one
    sequence, this number should be contained in the first (or only)
    track. If transfer of several multitrack sequences is required,
    this must be done as a group of format 1 files, each with a different
    sequence number.
"#]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Format<'a> {
    /// Format 0
    SingleMultiChannel,
    /// Format 1
    Simultaneous(Cow<'a, [u8; 2]>),
    /// Format 2
    SequentiallyIndependent(Cow<'a, [u8; 2]>),
}
impl<'a> Format<'a> {
    pub const fn single_multichannel() -> Self {
        Self::SingleMultiChannel
    }
    pub const fn simultaneous(bytes: &'a [u8; 2]) -> Self {
        Self::Simultaneous(Cow::Borrowed(bytes))
    }
    pub const fn sequentially_independent(bytes: &'a [u8; 2]) -> Self {
        Self::SequentiallyIndependent(Cow::Borrowed(bytes))
    }

    pub fn num_tracks(&self) -> u16 {
        use Format::*;
        match self {
            SingleMultiChannel => 1,
            Simultaneous(num) | SequentiallyIndependent(num) => u16::from_be_bytes(**num),
        }
    }
    pub const fn format_type(&self) -> FormatType {
        use Format::*;
        match self {
            SingleMultiChannel => FormatType::SingleMultiChannel,
            Simultaneous(_) => FormatType::Simultaneous,
            SequentiallyIndependent(_) => FormatType::SequentiallyIndependent,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatType {
    SingleMultiChannel,
    Simultaneous,
    SequentiallyIndependent,
}
