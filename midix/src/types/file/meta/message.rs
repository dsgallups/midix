use crate::prelude::*;

/// A "meta message", as defined by the SMF spec.
/// These events carry metadata about the track, such as tempo, time signature, copyright, etc...
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Meta {
    /// For `Format::Sequential` MIDI file types, `TrackNumber` can be empty, and defaults to
    /// the track index.
    TrackNumber(Option<u16>),
    /// Arbitrary text associated to an instant.
    Text(String),
    /// A copyright notice.
    Copyright(String),
    /// Information about the name of the track.
    TrackName(String),
    /// Information about the name of the current instrument.
    InstrumentName(String),
    /// Arbitrary lyric information associated to an instant.
    Lyric(String),
    /// Arbitrary marker text associated to an instant.
    Marker(String),
    /// Arbitrary cue point text associated to an instant.
    CuePoint(String),
    /// Information about the name of the current program.
    ProgramName(String),
    /// Name of the device that this file was intended to be played with.
    DeviceName(String),
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
    ///
    /// TODO
    SmpteOffset(Vec<u8>),
    /// In order of the MIDI specification, numerator, denominator, MIDI clocks per click, 32nd
    /// notes per quarter
    TimeSignature(TimeSignature),
    KeySignature(KeySignature),
    /// Arbitrary data intended for the sequencer.
    /// This data is never sent to a device.
    SequencerSpecific(Vec<u8>),
    /// An unknown or malformed meta-message.
    ///
    /// The first `u8` is the raw meta-message identifier byte.
    /// The slice is the actual payload of the meta-message.
    Unknown(u8, Vec<u8>),
}
