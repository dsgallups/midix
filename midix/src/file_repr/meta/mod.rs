#![doc = r#"
Contains types that deal with file ['MetaMessage']s
"#]

mod tempo;
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
    TrackNumber(&'a [u8]),
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
    CuePoint(&'a [u8]),
    /// Information about the name of the current program.
    ProgramName(BytesText<'a>),
    /// Name of the device that this file was intended to be played with.
    DeviceName(BytesText<'a>),
    /// Number of the MIDI channel that this file was intended to be played with.
    MidiChannel(ChannelId<'a>),
    /// Number of the MIDI port that this file was intended to be played with.
    MidiPort(u8),
    /// Obligatory at track end.
    EndOfTrack,
    /// Amount of microseconds per beat (quarter note).
    ///
    /// Usually appears at the beginning of a track, before any midi events are sent, but there
    /// are no guarantees.
    Tempo(Tempo<'a>),
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
    SequencerSpecific(&'a [u8]),
    /// An unknown or malformed meta-message.
    ///
    /// The first `u8` is the raw meta-message identifier byte.
    /// The slice is the actual payload of the meta-message.
    Unknown(&'a u8, &'a [u8]),
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
            0x01 => MetaMessage::Text(BytesText::new_from_byte_slice(data)?),
            0x02 => MetaMessage::Copyright(BytesText::new_from_byte_slice(data)?),
            0x03 => MetaMessage::TrackName(BytesText::new_from_byte_slice(data)?),
            0x04 => MetaMessage::InstrumentName(BytesText::new_from_byte_slice(data)?),
            0x05 => MetaMessage::Lyric(BytesText::new_from_byte_slice(data)?),
            0x06 => MetaMessage::Marker(BytesText::new_from_byte_slice(data)?),
            0x07 => MetaMessage::CuePoint(data),
            0x08 => MetaMessage::ProgramName(BytesText::new_from_byte_slice(data)?),
            0x09 => MetaMessage::DeviceName(BytesText::new_from_byte_slice(data)?),
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
                let c = data.first().unwrap();
                MetaMessage::MidiChannel(ChannelId::new(*c)?)
            }
            0x21 => {
                if data.len() != 1 {
                    return Err(inv_data(
                        reader,
                        format!("Varlen is invalid for port (should be 1, is {}", data.len()),
                    ));
                }
                let port = *reader.read_next()?;
                MetaMessage::MidiPort(port)
            }
            0x2F => MetaMessage::EndOfTrack,
            0x51 => {
                //FF 51 03 tttttt
                if data.len() != 3 {
                    return Err(inv_data(
                        reader,
                        format!(
                            "Varlen is invalid for tempo (should be 3, is {}",
                            data.len()
                        ),
                    ));
                }
                MetaMessage::Tempo(Tempo::new_from_byte_slice(data.try_into().unwrap()))
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
                MetaMessage::TimeSignature(TimeSignature::new_from_byte_slice(
                    data.try_into().unwrap(),
                ))
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
                MetaMessage::KeySignature(KeySignature::new_from_byte_slice(
                    data.try_into().unwrap(),
                ))
            }
            0x7F => MetaMessage::SequencerSpecific(data),
            _ => MetaMessage::Unknown(type_byte, data),
        })
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
