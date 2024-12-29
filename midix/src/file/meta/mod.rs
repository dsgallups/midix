#![doc = r#"
Contains types that deal with file [`Meta`] messages
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
#[derive(Clone, PartialEq, Debug)]
pub enum Meta<'a> {
    /// For `Format::Sequential` MIDI file types, `TrackNumber` can be empty, and defaults to
    /// the track index.
    TrackNumber(&'a [u8]),
    /// Arbitrary text associated to an instant.
    Text(Text<'a>),
    /// A copyright notice.
    Copyright(Text<'a>),
    /// Information about the name of the track.
    TrackName(Text<'a>),
    /// Information about the name of the current instrument.
    InstrumentName(Text<'a>),
    /// Arbitrary lyric information associated to an instant.
    Lyric(Text<'a>),
    /// Arbitrary marker text associated to an instant.
    Marker(&'a [u8]),
    /// Arbitrary cue point text associated to an instant.
    CuePoint(&'a [u8]),
    /// Information about the name of the current program.
    ProgramName(Text<'a>),
    /// Name of the device that this file was intended to be played with.
    DeviceName(Text<'a>),
    /// Number of the MIDI channel that this file was intended to be played with.
    MidiChannel(Channel<'a>),
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
impl<'a> Meta<'a> {
    pub(crate) fn read<'slc, 'r>(reader: &'r mut Reader<&'slc [u8]>) -> ReadResult<Self>
    where
        'slc: 'a,
    {
        let type_byte = reader.read_next()?;

        let data = reader.read_varlen_slice()?;

        Ok(match type_byte {
            0x00 => Meta::TrackNumber(data),
            0x01 => Meta::Text(Text::new_from_byte_slice(data)?),
            0x02 => Meta::Copyright(Text::new_from_byte_slice(data)?),
            0x03 => Meta::TrackName(Text::new_from_byte_slice(data)?),
            0x04 => Meta::InstrumentName(Text::new_from_byte_slice(data)?),
            0x05 => Meta::Lyric(Text::new_from_byte_slice(data)?),
            0x06 => Meta::Marker(data),
            0x07 => Meta::CuePoint(data),
            0x08 => Meta::ProgramName(Text::new_from_byte_slice(data)?),
            0x09 => Meta::DeviceName(Text::new_from_byte_slice(data)?),
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
                Meta::MidiChannel(Channel::new(*c)?)
            }
            0x21 => {
                if data.len() != 1 {
                    return Err(inv_data(
                        reader,
                        format!("Varlen is invalid for port (should be 1, is {}", data.len()),
                    ));
                }
                let port = *reader.read_next()?;
                Meta::MidiPort(port)
            }
            0x2F => Meta::EndOfTrack,
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
                Meta::Tempo(Tempo::new_from_byte_slice(data.try_into().unwrap()))
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
                Meta::TimeSignature(TimeSignature::new_from_byte_slice(data.try_into().unwrap()))
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
                Meta::KeySignature(KeySignature::new_from_byte_slice(data.try_into().unwrap()))
            }
            0x7F => Meta::SequencerSpecific(data),
            _ => Meta::Unknown(type_byte, data),
        })
    }

    /*pub fn to_owned(self) -> MetaMessage {
        use Meta::*;
        match self {
            TrackNumber(n) => match n.len() {
                2 => {
                    let n: [u8; 2] = n.try_into().unwrap();
                    MetaMessage::TrackNumber(Some(u16::from_be_bytes(n)))
                }
                _ => MetaMessage::TrackNumber(None),
            },
            Text(t) => {
                let v = String::from_utf8(t.to_vec()).unwrap_or_default();
                MetaMessage::Text(v)
            }
            Copyright(t) => {
                let v = String::from_utf8(t.to_vec()).unwrap_or_default();
                MetaMessage::Copyright(v)
            }
            TrackName(t) => {
                let v = String::from_utf8(t.to_vec()).unwrap_or_default();
                MetaMessage::TrackName(v)
            }
            InstrumentName(t) => {
                let v = String::from_utf8(t.to_vec()).unwrap_or_default();
                MetaMessage::InstrumentName(v)
            }
            Lyric(t) => {
                let v = String::from_utf8(t.to_vec()).unwrap_or_default();
                MetaMessage::Lyric(v)
            }
            Marker(t) => {
                let v = String::from_utf8(t.to_vec()).unwrap_or_default();
                MetaMessage::Marker(v)
            }
            CuePoint(t) => {
                let v = String::from_utf8(t.to_vec()).unwrap_or_default();
                MetaMessage::CuePoint(v)
            }
            ProgramName(t) => {
                let v = String::from_utf8(t.to_vec()).unwrap_or_default();
                MetaMessage::ProgramName(v)
            }
            DeviceName(t) => {
                let v = String::from_utf8(t.to_vec()).unwrap_or_default();
                MetaMessage::DeviceName(v)
            }
            MidiChannel(c) => MetaMessage::MidiChannel(c),
            MidiPort(p) => MetaMessage::MidiPort(p),
            EndOfTrack => MetaMessage::EndOfTrack,
            Tempo(_) => todo!(),
            SmpteOffset(_) => todo!(),
            TimeSignature(_) => todo!(),
            KeySignature(_) => todo!(),
            SequencerSpecific(s) => MetaMessage::SequencerSpecific(s.to_vec()),
            Unknown(r, d) => MetaMessage::Unknown(*r, d.to_vec()),
        }
    }*/
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
