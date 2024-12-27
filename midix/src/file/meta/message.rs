use crate::prelude::*;

use super::{KeySignature, TempoBorrowed, TimeSignature};

/// A "meta message", as defined by the SMF spec.
/// These events carry metadata about the track, such as tempo, time signature, copyright, etc...
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum MetaMessage<'a> {
    /// For `Format::Sequential` MIDI file types, `TrackNumber` can be empty, and defaults to
    /// the track index.
    TrackNumber(&'a [u8]),
    /// Arbitrary text associated to an instant.
    Text(&'a [u8]),
    /// A copyright notice.
    Copyright(&'a [u8]),
    /// Information about the name of the track.
    TrackName(&'a [u8]),
    /// Information about the name of the current instrument.
    InstrumentName(&'a [u8]),
    /// Arbitrary lyric information associated to an instant.
    Lyric(&'a [u8]),
    /// Arbitrary marker text associated to an instant.
    Marker(&'a [u8]),
    /// Arbitrary cue point text associated to an instant.
    CuePoint(&'a [u8]),
    /// Information about the name of the current program.
    ProgramName(&'a [u8]),
    /// Name of the device that this file was intended to be played with.
    DeviceName(&'a [u8]),
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
    Tempo(TempoBorrowed<'a>),
    /// The MIDI SMPTE offset meta message specifies an offset for the starting point of a MIDI
    /// track from the start of a sequence in terms of SMPTE time (hours:minutes:seconds:frames:subframes).
    ///
    /// [Reference](https://www.recordingblogs.com/wiki/midi-smpte-offset-meta-message)
    SmpteOffset(&'a [u8]),
    /// In order of the MIDI specification, numerator, denominator, MIDI clocks per click, 32nd
    /// notes per quarter
    TimeSignature(TimeSignature<'a>),
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
    pub fn read<'slc, 'r>(reader: &'r mut Reader<&'slc [u8]>) -> ReadResult<Self>
    where
        'slc: 'a,
    {
        let type_byte = reader.read_next()?;

        let data = reader.read_varlen()?;

        Ok(match type_byte {
            0x00 => MetaMessage::TrackNumber(data),
            0x01 => MetaMessage::Text(data),
            0x02 => MetaMessage::Copyright(data),
            0x03 => MetaMessage::TrackName(data),
            0x04 => MetaMessage::InstrumentName(data),
            0x05 => MetaMessage::Lyric(data),
            0x06 => MetaMessage::Marker(data),
            0x07 => MetaMessage::CuePoint(data),
            0x08 => MetaMessage::ProgramName(data),
            0x09 => MetaMessage::DeviceName(data),
            0x20 => {
                if data.len() != 1 {
                    return Err(ReaderError::invalid_data());
                }
                let c = u8::from_be_bytes(data.try_into().unwrap());
                let channel = Channel::new(c)?;
                MetaMessage::MidiChannel(channel)
            }
            0x21 => {
                if data.len() != 1 {
                    return Err(ReaderError::invalid_data());
                }
                let port = *reader.read_next()?;
                MetaMessage::MidiPort(port)
            }
            0x2F => MetaMessage::EndOfTrack,
            0x51 => {
                //FF 51 03 tttttt
                if data.len() != 3 {
                    return Err(ReaderError::invalid_data());
                }
                MetaMessage::Tempo(TempoBorrowed::new(data.try_into().unwrap()))
            }
            0x54 => {
                return Err(ReaderError::unimplemented(
                    "SMPTE Offset not yet implemented.",
                ))
            }
            0x58 if data.len() >= 4 => {
                //FF 58 04 nn dd cc bb
                if data.len() != 4 {
                    return Err(ReaderError::invalid_data());
                }
                MetaMessage::TimeSignature(TimeSignature::new(data.try_into().unwrap()))
            }
            0x59 => {
                if data.len() != 2 {
                    return Err(ReaderError::invalid_data());
                }
                MetaMessage::KeySignature(KeySignature::new(data.try_into().unwrap()))
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
