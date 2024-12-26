use crate::chunk::ReaderError;

use super::{convert_u32, ReadResult, Reader};

#[doc = r#"
The header chunk at the beginning of the file specifies some basic information about the data in the file. Here's the syntax of the complete chunk:

<Header Chunk> = <chunk type><length><format><ntrks><division>

As described above, <chunk type> is the four ASCII characters 'MThd'; <length> is a 32-bit representation of the number 6 (high byte first).

The data section contains three 16-bit words, stored most-significant byte first.

The first word, <format>, specifies the overall organisation of the file. Only three values of <format> are specified:

0-the file contains a single multi-channel track
1-the file contains one or more simultaneous tracks (or MIDI outputs) of a sequence
2-the file contains one or more sequentially independent single-track patterns

More information about these formats is provided below.

The next word, <ntrks>, is the number of track chunks in the file. It will always be 1 for a format 0 file.

The third word, <division>, specifies the meaning of the delta-times. It has two formats, one for metrical time, and one for time-code-based time:
bit 15 	bits 14 thru 8	bits 7 thru 0
0 	ticks per quarter-note
1 	negative SMPTE format 	ticks per frame

If bit 15 of <division> is zero, the bits 14 thru 0 represent the number of delta time "ticks" which make up a quarter-note. For instance, if division is 96, then a time interval of an eighth-note between two events in the file would be 48.

If bit 15 of <division> is a one, delta times in a file correspond to subdivisions of a second, in a way consistent with SMPTE and MIDI Time Code. Bits 14 thru 8 contain one of the four values -24, -25, -29, or -30, corresponding to the four standard SMPTE and MIDI Time Code formats (-29 corresponds to 30 drop frame), and represents the number of frames per second. These negative numbers are stored in two's compliment form. The second byte (stored positive) is the resolution within a frame: typical values may be 4 (MIDI Time Code resolution), 8, 10, 80 (bit resolution), or 100. This stream allows exact specifications of time-code-based tracks, but also allows millisecond-based tracks by specifying 25 frames/sec and a resolution of 40 units per frame. If the events in a file are stored with a bit resolution of thirty-frame time code, the division word would be E250 hex.
"#]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MidiHeader<'a> {
    length: &'a [u8; 4],
    format: MidiFormat<'a>,
    timing: MidiTiming<'a>,
}

impl<'a> MidiHeader<'a> {
    /// Assumes that the chunk type bytes ("MThd") have ALREADY been read
    pub fn read<'slc, 'r>(reader: &'r mut Reader<&'slc [u8]>) -> ReadResult<Self>
    where
        'slc: 'a,
    {
        let length: &[u8; 4] = reader.read_exact_size()?;
        let format_bytes: &[u8; 2] = reader.read_exact_size()?;
        let num_tracks: &[u8; 2] = reader.read_exact_size()?;

        let format = match format_bytes[1] {
            0 => {
                if num_tracks[1] != 1 {
                    return Err(ReaderError::invalid_data());
                }
                MidiFormat::SingleMultiChannel
            } // Always 1 track
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
    pub fn length(self) -> u32 {
        convert_u32(self.length)
    }
    pub fn format(&self) -> u8 {
        todo!()
    }
    pub fn format_type(&self) -> MidiFormatType {
        use MidiFormat::*;
        match self.format {
            SingleMultiChannel => MidiFormatType::SingleMultiChannel,
            Simultaneous(_) => MidiFormatType::Simultaneous,
            SequentiallyIndependent(_) => MidiFormatType::SequentiallyIndependent,
        }
    }
    pub fn num_tracks(&self) -> u16 {
        self.format.num_tracks()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiFormatType {
    SingleMultiChannel,
    Simultaneous,
    SequentiallyIndependent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiFormat<'a> {
    SingleMultiChannel,
    Simultaneous(&'a [u8; 2]),
    SequentiallyIndependent(&'a [u8; 2]),
}

impl MidiFormat<'_> {
    pub fn num_tracks(self) -> u16 {
        use MidiFormat::*;
        match self {
            SingleMultiChannel => 1,
            Simultaneous(num) | SequentiallyIndependent(num) => u16::from_be_bytes(*num),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiTiming<'a> {
    TicksPerQuarterNote(&'a [u8; 2]),
}

impl<'a> MidiTiming<'a> {
    /// Assumes the next two bytes are for a midi division.
    pub fn read<'r, 'slc>(reader: &'r mut Reader<&'slc [u8]>) -> ReadResult<Self>
    where
        'slc: 'a,
    {
        let bytes: &[u8; 2] = reader.read_exact_size()?;
        match bytes[0] >> 7 {
            0 => {
                //this is ticks per quarter_note
                Ok(MidiTiming::TicksPerQuarterNote(bytes))
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

#[test]
fn read_midi_header_simultaneous() {
    let bytes = [
        0x00, 0x00, 0x00, 0x06, //length
        0x00, 0x01, //format
        0x00, 0x03, //num_tracks
        0x00, 0x78, //timing
    ];
    let mut reader = Reader::from_byte_slice(&bytes);

    let result = MidiHeader::read(&mut reader).unwrap();

    assert_eq!(result.length(), 6);
    assert_eq!(result.format_type(), MidiFormatType::Simultaneous);
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

    let result = MidiHeader::read(&mut reader).unwrap();

    assert_eq!(result.length(), 6);
    assert_eq!(result.format_type(), MidiFormatType::SingleMultiChannel);
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

    let err = MidiHeader::read(&mut reader).expect_err("Invalid");
    assert!(matches!(err, ReaderError::Io(_)))
}
