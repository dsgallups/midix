mod timing;
pub use timing::*;

use crate::prelude::*;

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
pub struct MidiHeaderRef<'a> {
    format: MidiFormatRef<'a>,
    timing: MidiTimingRef<'a>,
}

impl<'a> MidiHeaderRef<'a> {
    /// Assumes that the chunk type bytes ("MThd") have ALREADY been read
    pub fn read<'slc, 'r>(reader: &'r mut OldReader<&'slc [u8]>) -> OldReadResult<Self>
    where
        'slc: 'a,
    {
        let length = u32::from_be_bytes(*reader.read_exact_size()?);
        if length != 6 {
            return Err(OldReaderError::invalid_data());
        }

        let format_bytes: &[u8; 2] = reader.read_exact_size()?;
        let num_tracks: &[u8; 2] = reader.read_exact_size()?;

        let format = match format_bytes[1] {
            0 => {
                if num_tracks[1] != 1 {
                    return Err(OldReaderError::invalid_data());
                }
                MidiFormatRef::SingleMultiChannel
            } // Always 1 track
            1 => MidiFormatRef::Simultaneous(num_tracks),
            2 => MidiFormatRef::SequentiallyIndependent(num_tracks),
            _ => return Err(OldReaderError::invalid_input("Invalid MIDI format")),
        };

        let timing = MidiTimingRef::read(reader)?;

        Ok(Self { format, timing })
    }
    pub const fn length(self) -> u32 {
        6
    }
    pub fn format(&self) -> MidiFormatRef<'a> {
        self.format
    }
    pub fn format_type(&self) -> MidiFormatType {
        self.format.format_type()
    }
    pub fn num_tracks(&self) -> u16 {
        self.format.num_tracks()
    }
    pub fn timing(&self) -> MidiTimingRef {
        self.timing
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
    let mut reader = OldReader::from_byte_slice(&bytes);

    let result = MidiHeaderRef::read(&mut reader).unwrap();

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
    let mut reader = OldReader::from_byte_slice(&bytes);

    let result = MidiHeaderRef::read(&mut reader).unwrap();

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
    let mut reader = OldReader::from_byte_slice(&bytes);

    let err = MidiHeaderRef::read(&mut reader).expect_err("Invalid");
    assert!(matches!(err, OldReaderError::Io(_)))
}
