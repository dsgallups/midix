use reader::MidiSource;

use crate::prelude::*;

#[doc = r#"
The header chunk at the beginning of the file specifies some basic information about
the data in the file. Here's the syntax of the complete chunk:

`<Header Chunk> = <chunk type><length><format><ntrks><division>`

As described above, `<chunk type>` is the four ASCII characters 'MThd'; `<length>`
is a 32-bit representation of the number 6 (high byte first).

The data section contains three 16-bit words, stored most-significant byte first.

The first word, `<format>`, specifies the overall organisation of the file. Only
three values of `<format>` are specified:

0-the file contains a single multi-channel track
1-the file contains one or more simultaneous tracks (or MIDI outputs) of a sequence
2-the file contains one or more sequentially independent single-track patterns

More information about these formats is provided below.

The next word, `<ntrks>` is the number of track chunks in the file. It will always
be 1 for a format 0 file.

The third word, `<division>`, specifies the meaning of the delta-times. It has two
formats, one for metrical time, and one for time-code-based time:

| bit 15 | bits 14 thru 8 | bits 7 thru 0 |
|--------|----------------|---------------|
|0       | ticks per quarter-note | ticks per quarter-note cont. |
|1       |negative SMPTE format | ticks per frame |

If bit 15 of `<division>` is zero, the bits 14 thru 0 represent the number of delta time "ticks" which make up a quarter-note. For instance, if division is 96, then a time interval of an eighth-note between two events in the file would be 48.

If bit 15 of `<division>` is a one, delta times in a file correspond to subdivisions of a second, in a way consistent with SMPTE and MIDI Time Code. Bits 14 thru 8 contain one of the four values -24, -25, -29, or -30, corresponding to the four standard SMPTE and MIDI Time Code formats (-29 corresponds to 30 drop frame), and represents the number of frames per second. These negative numbers are stored in two's compliment form. The second byte (stored positive) is the resolution within a frame: typical values may be 4 (MIDI Time Code resolution), 8, 10, 80 (bit resolution), or 100. This stream allows exact specifications of time-code-based tracks, but also allows millisecond-based tracks by specifying 25 frames/sec and a resolution of 40 units per frame. If the events in a file are stored with a bit resolution of thirty-frame time code, the division word would be E250 hex.
"#]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawHeaderChunk<'a> {
    format: RawFormat<'a>,
    timing: Timing<'a>,
}

impl<'a> RawHeaderChunk<'a> {
    /// Assumes that the chunk type bytes ("MThd") have ALREADY been read
    pub(crate) fn read<'slc, 'r, R>(reader: &'r mut Reader<R>) -> ReadResult<Self>
    where
        R: MidiSource<'slc>,
        'slc: 'a,
    {
        let length = u32::from_be_bytes(*reader.read_exact_size()?);
        if length != 6 {
            return Err(inv_data(reader, "Length of header chunk is not 6"));
        }

        let format_bytes: BytesConst<'_, 2> = reader.read_exact_size()?;
        let num_tracks: BytesConst<'_, 2> = reader.read_exact_size()?;

        let format = match format_bytes[1] {
            0 => {
                if num_tracks[1] != 1 {
                    return Err(inv_data(
                        reader,
                        "Type 0 MIDI format (SingleMultiChannel) defines multiple tracks!",
                    ));
                };
                RawFormat::single_multichannel()
            } // Always 1 track
            1 => RawFormat::simultaneous_from_byte_slice(num_tracks),
            2 => RawFormat::sequentially_independent_from_byte_slice(num_tracks),
            t => return Err(inv_data(reader, format!("Invalid MIDI format {}", t))),
        };

        let timing = Timing::read(reader)?;

        Ok(Self { format, timing })
    }

    /// Get the length of the header. This is ALWAYS 6.
    #[allow(clippy::len_without_is_empty)]
    pub const fn len(&self) -> u32 {
        6
    }

    /// Get the describing format defined by the header. Includes information about the number
    /// of tracks identified.
    pub fn format(&self) -> &RawFormat<'a> {
        &self.format
    }

    /// Get the describing format type by the header
    ///
    /// identified as `<format>` in the docs
    pub fn format_type(&self) -> FormatType {
        self.format.format_type()
    }

    /// Get the number of tracks identified in the header
    ///
    /// identified as `<ntrks>` in the docs
    pub fn num_tracks(&self) -> u16 {
        self.format.num_tracks()
    }
    /// Get the timing property of the header
    ///
    /// identified as `<division>` in the docs
    pub fn timing(&self) -> &Timing {
        &self.timing
    }
}

/// The header timing type.
///
/// This is either the number of ticks per quarter note or
/// the alternative SMTPE format. See the [`RawHeaderChunk`] docs for more information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Timing<'a> {
    /// The midi file's delta times are defined using a tick rate per quarter note
    TicksPerQuarterNote(BytesConst<'a, 2>),

    /// The midi file's delta times are defined using an SMPTE and MIDI Time Code
    NegativeSmpte(BytesConst<'a, 2>),
}

impl<'a> Timing<'a> {
    /// The tickrate per quarter note defines what a "quarter note" means.
    ///
    /// The leading bit of the u16 is disregarded, so 1-32767
    pub fn new_ticks_per_quarter_note(tpqn: u16) -> Self {
        let msb = (tpqn >> 8) as u8;
        let lsb = (tpqn & 0x00FF) as u8;
        Self::TicksPerQuarterNote([msb, lsb].into())
    }

    pub(crate) fn read<'slc, 'r, R>(reader: &'r mut Reader<R>) -> ReadResult<Self>
    where
        R: MidiSource<'slc>,
        'slc: 'a,
    {
        let bytes: BytesConst<'_, 2> = reader.read_exact_size()?;
        match bytes[0] >> 7 {
            0 => {
                //this is ticks per quarter_note
                Ok(Timing::TicksPerQuarterNote(bytes))
            }
            1 => Ok(Timing::NegativeSmpte(bytes)),
            t => Err(inv_data(reader, format!("Invalid MIDI Timing type {}", t))),
        }
    }
    /// Returns Some if the midi timing is defined
    /// as ticks per quarter note
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

#[test]
fn ensure_timing_encoding_of_tpqn() {
    assert_eq!(
        Some(71),
        Timing::new_ticks_per_quarter_note(71).ticks_per_quarter_note()
    )
}

#[test]
fn read_midi_header_simultaneous() {
    let bytes = [
        0x00, 0x00, 0x00, 0x06, //length
        0x00, 0x01, //format
        0x00, 0x03, //num_tracks
        0x00, 0x78, //timing
    ];
    let mut reader = Reader::from_byte_slice(&bytes);

    let result = RawHeaderChunk::read(&mut reader).unwrap();

    assert_eq!(result.len(), 6);
    assert_eq!(result.format_type(), FormatType::Simultaneous);
    assert_eq!(result.num_tracks(), 3);
}

#[test]
fn read_midi_header_single_multichannel() {
    let bytes = [
        0x00, 0x00, 0x00, 0x06, //length
        0x00, 0x00, //format
        0x00, 0x01, //num_tracks
        0x00, 0x78, //timing
    ];
    let mut reader = Reader::from_byte_slice(&bytes);

    let result = RawHeaderChunk::read(&mut reader).unwrap();

    assert_eq!(result.len(), 6);
    assert_eq!(result.format_type(), FormatType::SingleMultiChannel);
    assert_eq!(result.num_tracks(), 1);
}

#[test]
fn read_midi_header_single_multichannel_invalid() {
    let bytes = [
        0x00, 0x00, 0x00, 0x06, //length
        0x00, 0x00, //format
        0x00, 0x03, //num_tracks
        0x00, 0x78, //timing
    ];
    let mut reader = Reader::from_byte_slice(&bytes);

    let _err = RawHeaderChunk::read(&mut reader).expect_err("Invalid");
    //assert!(matches!(err, ReaderError::Io(_)))
}
