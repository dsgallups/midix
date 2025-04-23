use std::iter;

use bevy::asset::uuid::Uuid;
use midix::prelude::*;

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
pub(crate) enum SinkCommand {
    /// Play a new song
    NewSong {
        /// What kind of song is this?
        song_type: SongType,
        /// The associated events with the song
        commands: Vec<Timed<ChannelVoiceMessage>>,
    },
    /// Stop a song
    Stop {
        song_id: Option<SongId>,
        stop_voices: bool,
    },
}

/// A set of commands
#[derive(Clone, Debug)]
pub struct MidiSong {
    pub(crate) id: SongId,
    pub(crate) events: Vec<Timed<ChannelVoiceMessage>>,
    pub(crate) looped: bool,
}

impl MidiSong {
    /// Create a set of commands
    pub fn new(events: Vec<Timed<ChannelVoiceMessage>>) -> Self {
        Self {
            id: SongId::default(),
            events,
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
        self.events
            .iter_mut()
            .for_each(|cmd| cmd.timestamp = (cmd.timestamp as f64 * speed) as u64);
        self
    }

    /// Returns the all timed midi events for the song.
    ///
    /// Not guaranteed to be sorted.
    pub fn events(&self) -> &[Timed<ChannelVoiceMessage>] {
        &self.events
    }
}

impl SongWriter for Timed<ChannelVoiceMessage> {
    fn events(&self) -> impl Iterator<Item = Timed<ChannelVoiceMessage>> {
        iter::once(*self)
    }
}
impl SongWriter for Vec<Timed<ChannelVoiceMessage>> {
    fn events(&self) -> impl Iterator<Item = Timed<ChannelVoiceMessage>> {
        self.iter().copied()
    }
}
impl SongWriter for [Timed<ChannelVoiceMessage>] {
    fn events(&self) -> impl Iterator<Item = Timed<ChannelVoiceMessage>> {
        self.iter().copied()
    }
}
