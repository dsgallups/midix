#![doc = r#"
Contains types that deal with file ['MetaMessage']s
"#]

mod tempo;
use num_enum::TryFromPrimitive;
pub use tempo::*;
mod time_signature;
pub use time_signature::*;
mod key_signature;
pub use key_signature::*;
mod text;
pub use text::*;

use crate::prelude::*;
/// A "meta message", as defined by the SMF spec.
/// These events carry metadata about the track, such as tempo, time signature, copyright, etc...
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MetaMessage<'a> {
    /// For `Format::Sequential` MIDI file types, `TrackNumber` can be empty, and defaults to
    /// the track index.
    TrackNumber(Bytes<'a>),
    /// Arbitrary text associated to an instant.
    Text(BytesText<'a>),
    /// A copyright notice.
    Copyright(BytesText<'a>),
    /// Information about the name of the track.
    TrackName(BytesText<'a>),
    /// Information about the name of the current instrument.
    InstrumentName(BytesText<'a>),
    /// Arbitrary lyric information associated to an instant.
    Lyric(BytesText<'a>),
    /// Arbitrary marker text associated to an instant.
    Marker(BytesText<'a>),
    /// Arbitrary cue point text associated to an instant.
    CuePoint(Bytes<'a>),
    /// Information about the name of the current program.
    ProgramName(BytesText<'a>),
    /// Name of the device that this file was intended to be played with.
    DeviceName(BytesText<'a>),
    /// Number of the MIDI channel that this file was intended to be played with.
    MidiChannel(Channel),
    /// Number of the MIDI port that this file was intended to be played with.
    MidiPort(u8),
    /// Obligatory at track end.
    EndOfTrack,
    /// Amount of microseconds per beat (quarter note).
    ///
    /// Usually appears at the beginning of a track, before any midi events are sent, but there
    /// are no guarantees.
    Tempo(Tempo),
    /// The MIDI SMPTE offset meta message specifies an offset for the starting point of a MIDI
    /// track from the start of a sequence in terms of SMPTE time (hours:minutes:seconds:frames:subframes).
    ///
    /// [Reference](https://www.recordingblogs.com/wiki/midi-smpte-offset-meta-message)
    SmpteOffset(&'a [u8]),
    /// In order of the MIDI specification, numerator, denominator, MIDI clocks per click, 32nd
    /// notes per quarter
    TimeSignature(TimeSignature<'a>),
    /// An event defining the key signature of the track
    KeySignature(KeySignature<'a>),
    /// Arbitrary data intended for the sequencer.
    /// This data is never sent to a device.
    SequencerSpecific(Bytes<'a>),
    /// An unknown or malformed meta-message.
    ///
    /// The first `u8` is the raw meta-message identifier byte.
    /// The slice is the actual payload of the meta-message.
    Unknown(u8, Bytes<'a>),
}
impl<'a> MetaMessage<'a> {
    pub(crate) fn read<'slc, 'r, R>(reader: &'r mut Reader<R>) -> ReadResult<Self>
    where
        R: MidiSource<'slc>,
        'slc: 'a,
    {
        let type_byte = reader.read_next()?;
        let data = reader.read_varlen_slice()?;

        Ok(match type_byte {
            0x00 => MetaMessage::TrackNumber(data),
            0x01 => MetaMessage::Text(BytesText::new_from_bytes(data)?),
            0x02 => MetaMessage::Copyright(BytesText::new_from_bytes(data)?),
            0x03 => MetaMessage::TrackName(BytesText::new_from_bytes(data)?),
            0x04 => MetaMessage::InstrumentName(BytesText::new_from_bytes(data)?),
            0x05 => MetaMessage::Lyric(BytesText::new_from_bytes(data)?),
            0x06 => MetaMessage::Marker(BytesText::new_from_bytes(data)?),
            0x07 => MetaMessage::CuePoint(data),
            0x08 => MetaMessage::ProgramName(BytesText::new_from_bytes(data)?),
            0x09 => MetaMessage::DeviceName(BytesText::new_from_bytes(data)?),
            0x20 => {
                if data.len() != 1 {
                    return Err(inv_data(
                        reader,
                        format!(
                            "Varlen is invalid for this channel (should be 1, is {}",
                            data.len()
                        ),
                    ));
                }
                //TODO: need to test thsi
                let c = data.first().unwrap();
                MetaMessage::MidiChannel(Channel::try_from_primitive(*c)?)
            }
            0x21 => {
                if data.len() != 1 {
                    return Err(inv_data(
                        reader,
                        format!("Varlen is invalid for port (should be 1, is {}", data.len()),
                    ));
                }
                let port = reader.read_next()?;
                MetaMessage::MidiPort(port)
            }
            0x2F => MetaMessage::EndOfTrack,
            0x51 => {
                //FF 51 03 tttttt
                MetaMessage::Tempo(Tempo::new_from_bytes(data))
            }
            0x54 => {
                //TODO
                todo!("implement SMTPE")
                //return Err(inv_data(reader, "SMTPE is not yet implemented"));
            }
            0x58 if data.len() >= 4 => {
                //FF 58 04 nn dd cc bb
                if data.len() != 4 {
                    return Err(inv_data(
                        reader,
                        format!(
                            "Varlen is invalid for time signature (should be 4, is {}",
                            data.len()
                        ),
                    ));
                }
                MetaMessage::TimeSignature(TimeSignature::new_from_bytes(data.try_into().unwrap()))
            }
            0x59 => {
                if data.len() != 2 {
                    return Err(inv_data(
                        reader,
                        format!(
                            "Varlen is invalid for key signature (should be 2, is {}",
                            data.len()
                        ),
                    ));
                }
                MetaMessage::KeySignature(KeySignature::new_from_bytes(data.try_into().unwrap()))
            }
            0x7F => MetaMessage::SequencerSpecific(data),
            _ => MetaMessage::Unknown(type_byte, data),
        })
    }

    /// Mutates the data of a track
    pub fn adjust_track_info(self, info: &mut TrackInfo<'a>) {
        use MetaMessage::*;

        match self {
            TrackName(name) => {
                info.name = Some(name);
            }
            DeviceName(device) => info.device = Some(device),
            MidiChannel(channel) => info.channel = Some(channel),
            Tempo(tempo) => info.tempo = tempo,
            _ => {}
        }
    }
}

/*#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct SmpteTime {
    hour: u8,
    minute: u8,
    second: u8,
    frame: u8,
    subframe: u8,
    fps: Fps,
}*/
