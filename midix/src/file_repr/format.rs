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
pub enum RawFormat {
    /// Format 0
    SingleMultiChannel,
    /// Format 1
    Simultaneous([u8; 2]),
    /// Format 2
    SequentiallyIndependent([u8; 2]),
}
impl RawFormat {
    /// Create a [`RawFormat::SingleMultiChannel`]
    pub const fn single_multichannel() -> Self {
        Self::SingleMultiChannel
    }

    /// Create a [`Format::Simultaneous`]
    pub(crate) const fn simultaneous_from_byte_slice(bytes: [u8; 2]) -> Self {
        Self::Simultaneous(bytes)
    }

    /// Create a [`Format::SequentiallyIndependent`]
    pub(crate) const fn sequentially_independent_from_byte_slice(bytes: [u8; 2]) -> Self {
        Self::SequentiallyIndependent(bytes)
    }

    /// Returns the number of tracks identified by the format.
    ///
    /// [`RawFormat::SingleMultiChannel`] will always return 1.
    pub const fn num_tracks(&self) -> u16 {
        use RawFormat::*;
        match &self {
            SingleMultiChannel => 1,
            Simultaneous(num) | SequentiallyIndependent(num) => u16::from_be_bytes(*num),
        }
    }

    /// Returns the format type of the format.
    pub const fn format_type(&self) -> FormatType {
        use RawFormat::*;
        match self {
            SingleMultiChannel => FormatType::SingleMultiChannel,
            Simultaneous(_) => FormatType::Simultaneous,
            SequentiallyIndependent(_) => FormatType::SequentiallyIndependent,
        }
    }
}

#[doc = r#"
Identifies the type of the MIDI file.

# Layout

A Format 0 file has a header chunk followed by one track chunk.
It is the most interchangeable representation of data. It is very
useful for a simple single-track player in a program which needs
to make synthesisers make sounds, but which is primarily concerned
with something else such as mixers or sound effect boxes. It is very
desirable to be able to produce such a format, even if your program
is track-based, in order to work with these simple programs.

A Format 1 or 2 file has a header chunk followed by one or more
track chunks. programs which support several simultaneous tracks
should be able to save and read data in format 1, a vertically one
dimensional form, that is, as a collection of tracks. Programs which
support several independent patterns should be able to save and read
data in format 2, a horizontally one dimensional form. Providing these
minimum capabilities will ensure maximum interchangeability.

In a MIDI system with a computer and a SMPTE synchroniser which uses
Song Pointer and Timing Clock, tempo maps (which describe the tempo
throughout the track, and may also include time signature information,
so that the bar number may be derived) are generally created on the
computer. To use them with the synchroniser, it is necessary to transfer
them from the computer. To make it easy for the synchroniser to extract
this data from a MIDI File, tempo information should always be stored
in the first MTrk chunk. For a format 0 file, the tempo will be
scattered through the track and the tempo map reader should ignore the
intervening events; for a format 1 file, the tempo map must be stored
as the first track. It is polite to a tempo map reader to offer your
user the ability to make a format 0 file with just the tempo, unless
you can use format 1.

All MIDI Files should specify tempo and time signature. If they don't,
the time signature is assumed to be 4/4, and the tempo 120 beats per minute.
In format 0, these meta-events should occur at least at the beginning of the
single multi-channel track. In format 1, these meta-events should be contained
in the first track. In format 2, each of the temporally independent patterns
should contain at least initial time signature and tempo information.

Format IDs to support other structures may be defined in the future. A program
encountering an unknown format ID may still read other MTrk chunks it finds from
the file, as format 1 or 2, if its user can make sense of them and arrange
them into some other structure if appropriate. Also, more parameters may be
added to the MThd chunk in the future: it is important to read and honour the
length, even if it is longer than 6.

"#]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatType {
    /// Format 0
    SingleMultiChannel,

    /// Format 1
    Simultaneous,

    /// Format 2
    SequentiallyIndependent,
}
