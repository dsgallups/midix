use std::{iter, time::Duration};

use bevy::asset::uuid::Uuid;
use midix::prelude::ChannelVoiceMessage;

use crate::synth::SongWriter;

/// The identifier of a certain midi song
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct SongId(Uuid);

impl Default for SongId {
    fn default() -> Self {
        SongId(Uuid::new_v4())
    }
}
/// The type of song that will be sent to the synth
#[derive(Clone, Copy)]
pub enum SongType {
    /// No identifier, and therefore, no looping
    Anonymous,
    /// An identifier, and therefore, looping
    Identified {
        /// The song identifer
        id: SongId,
        /// true if it loops
        looped: bool,
    },
}

impl SongType {
    pub(crate) fn id(&self) -> Option<SongId> {
        match self {
            SongType::Anonymous => None,
            SongType::Identified { id, .. } => Some(*id),
        }
    }
}

/// Command the sink to do something
pub enum SinkCommand {
    /// Play a new song
    NewSong {
        /// What kind of song is this?
        song_type: SongType,
        /// The associated events with the song
        commands: Vec<TimedMidiEvent>,
    },
    /// Stop a song
    Stop(SongId),
}

/// A set of commands
#[derive(Clone, Debug)]
pub struct MidiSong {
    pub(crate) id: SongId,
    pub(crate) commands: Vec<TimedMidiEvent>,
    pub(crate) looped: bool,
}

impl MidiSong {
    /// Create a set of commands
    pub fn new(commands: Vec<TimedMidiEvent>) -> Self {
        Self {
            id: SongId::default(),
            commands,
            looped: false,
        }
    }
    /// Commands should be looped
    pub fn set_looped(mut self) -> Self {
        self.looped = true;
        self
    }
    /// set's the speed of the commands. Not absolute.
    pub fn set_speed(mut self, speed: f64) -> Self {
        let speed = 1. / speed;
        self.commands
            .iter_mut()
            .for_each(|cmd| cmd.timestamp = (cmd.timestamp as f64 * speed) as u64);
        self
    }
}

/// Send a command to the synth to play a note
#[derive(Copy, Clone, Debug)]
pub struct TimedMidiEvent {
    /// Micros
    pub(crate) timestamp: u64,
    pub(crate) event: ChannelVoiceMessage,
}
impl TimedMidiEvent {
    /// Create a command to play a note to the synth.
    ///
    /// Timestamp is delta micros from now.
    pub fn new(timestamp: u64, event: ChannelVoiceMessage) -> Self {
        Self { timestamp, event }
    }

    /// Use a duration to create a timed midi event
    pub fn new_from_duration(duration: Duration, event: ChannelVoiceMessage) -> Self {
        Self {
            timestamp: duration.as_micros() as u64,
            event,
        }
    }
}

impl SongWriter for TimedMidiEvent {
    fn commands(&self) -> impl Iterator<Item = TimedMidiEvent> {
        iter::once(*self)
    }
}
impl SongWriter for Vec<TimedMidiEvent> {
    fn commands(&self) -> impl Iterator<Item = TimedMidiEvent> {
        self.iter().copied()
    }
}
impl SongWriter for [TimedMidiEvent] {
    fn commands(&self) -> impl Iterator<Item = TimedMidiEvent> {
        self.iter().copied()
    }
}
