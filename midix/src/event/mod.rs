//! All sort of events and their parsers.
mod system;
pub use system::*;

use crate::{channel::Channel, primitive::SmpteTime, ChannelVoiceMessage};

/// Represents a parsed SMF track event.
///
/// Consists of a delta time (in MIDI ticks relative to the previous event) and the actual track
/// event.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct TrackEvent<'a> {
    /// How many MIDI ticks after the previous event should this event fire.
    pub delta: u32,
    /// The type of event along with event-specific data.
    pub kind: TrackEventKind<'a>,
}

/// Represents the different kinds of SMF events and their associated data.
///
/// It notably does *not* include the timing of the event; the `TrackEvent` struct is responsible
/// for this.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum TrackEventKind<'a> {
    /// A message associated to a MIDI channel carrying musical data.
    ///
    /// Usually, the bulk of MIDI data is these kind of messages.
    Midi(ChannelVoiceMessage),
    /// A System Exclusive message, carrying arbitrary data.
    ///
    /// The data bytes included here do not include the implicit `0xF0` prefix.
    ///
    /// Usually SysEx events end with an `0xF7` byte, but SysEx events that are split into several
    /// small packets may only contain the `0xF7` byte in the last packet fragment.
    SysEx(&'a [u8]),
    /// An escape sequence, intended to send arbitrary data to the MIDI synthesizer.
    Escape(&'a [u8]),
    /// A meta-message, giving extra information for correct playback, like tempo, song name,
    /// lyrics, etc...
    Meta(MetaMessage<'a>),
}

/// A "meta message", as defined by the SMF spec.
/// These events carry metadata about the track, such as tempo, time signature, copyright, etc...
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum MetaMessage<'a> {
    /// For `Format::Sequential` MIDI file types, `TrackNumber` can be empty, and defaults to
    /// the track index.
    TrackNumber(Option<u16>),
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
    Tempo(u32),
    /// The MIDI SMPTE offset meta message specifies an offset for the starting point of a MIDI
    /// track from the start of a sequence in terms of SMPTE time (hours:minutes:seconds:frames:subframes).
    ///
    /// [Reference](https://www.recordingblogs.com/wiki/midi-smpte-offset-meta-message)
    SmpteOffset(SmpteTime),
    /// In order of the MIDI specification, numerator, denominator, MIDI clocks per click, 32nd
    /// notes per quarter
    TimeSignature(u8, u8, u8, u8),
    /// As in the MIDI specification, negative numbers indicate number of flats and positive
    /// numbers indicate number of sharps.
    /// `false` indicates a major scale, `true` indicates a minor scale.
    KeySignature(i8, bool),
    /// Arbitrary data intended for the sequencer.
    /// This data is never sent to a device.
    SequencerSpecific(&'a [u8]),
    /// An unknown or malformed meta-message.
    ///
    /// The first `u8` is the raw meta-message identifier byte.
    /// The slice is the actual payload of the meta-message.
    Unknown(u8, &'a [u8]),
}
