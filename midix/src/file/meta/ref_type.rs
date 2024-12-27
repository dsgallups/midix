use crate::prelude::*;
/// A "meta message", as defined by the SMF spec.
/// These events carry metadata about the track, such as tempo, time signature, copyright, etc...
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum MetaRef<'a> {
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
    MidiChannel(&'a u8),
    /// Number of the MIDI port that this file was intended to be played with.
    MidiPort(u8),
    /// Obligatory at track end.
    EndOfTrack,
    /// Amount of microseconds per beat (quarter note).
    ///
    /// Usually appears at the beginning of a track, before any midi events are sent, but there
    /// are no guarantees.
    Tempo(TempoRef<'a>),
    /// The MIDI SMPTE offset meta message specifies an offset for the starting point of a MIDI
    /// track from the start of a sequence in terms of SMPTE time (hours:minutes:seconds:frames:subframes).
    ///
    /// [Reference](https://www.recordingblogs.com/wiki/midi-smpte-offset-meta-message)
    SmpteOffset(&'a [u8]),
    /// In order of the MIDI specification, numerator, denominator, MIDI clocks per click, 32nd
    /// notes per quarter
    TimeSignature(TimeSignatureRef<'a>),
    KeySignature(KeySignatureRef<'a>),
    /// Arbitrary data intended for the sequencer.
    /// This data is never sent to a device.
    SequencerSpecific(&'a [u8]),
    /// An unknown or malformed meta-message.
    ///
    /// The first `u8` is the raw meta-message identifier byte.
    /// The slice is the actual payload of the meta-message.
    Unknown(&'a u8, &'a [u8]),
}
impl<'a> MetaRef<'a> {
    pub fn read<'slc, 'r>(reader: &'r mut Reader<&'slc [u8]>) -> ReadResult<Self>
    where
        'slc: 'a,
    {
        let type_byte = reader.read_next()?;

        let data = reader.read_varlen_slice()?;

        Ok(match type_byte {
            0x00 => MetaRef::TrackNumber(data),
            0x01 => MetaRef::Text(data),
            0x02 => MetaRef::Copyright(data),
            0x03 => MetaRef::TrackName(data),
            0x04 => MetaRef::InstrumentName(data),
            0x05 => MetaRef::Lyric(data),
            0x06 => MetaRef::Marker(data),
            0x07 => MetaRef::CuePoint(data),
            0x08 => MetaRef::ProgramName(data),
            0x09 => MetaRef::DeviceName(data),
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
                //TODO: Need to check if it's a u4
                let c = data.first().unwrap();
                MetaRef::MidiChannel(c)
            }
            0x21 => {
                if data.len() != 1 {
                    return Err(inv_data(
                        reader,
                        format!("Varlen is invalid for port (should be 1, is {}", data.len()),
                    ));
                }
                let port = *reader.read_next()?;
                MetaRef::MidiPort(port)
            }
            0x2F => MetaRef::EndOfTrack,
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
                MetaRef::Tempo(TempoRef::new(data.try_into().unwrap()))
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
                MetaRef::TimeSignature(TimeSignatureRef::new(data.try_into().unwrap()))
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
                MetaRef::KeySignature(KeySignatureRef::new(data.try_into().unwrap()))
            }
            0x7F => MetaRef::SequencerSpecific(data),
            _ => MetaRef::Unknown(type_byte, data),
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
