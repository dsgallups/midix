#![doc = r#"
Components to make songs programatically
"#]

pub mod simple;
use bevy_platform::prelude::*;
pub use simple::*;

mod song_writer;
pub use song_writer::*;

mod builder;
pub use builder::*;

use crate::prelude::*;
use ::bevy::asset::uuid::Uuid;

/// The identifier of a certain midi song
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct SongId(Uuid);

impl Default for SongId {
    fn default() -> Self {
        SongId(Uuid::new_v4())
    }
}

/// A set of commands
#[derive(Clone, Debug)]
pub struct MidiSong {
    pub(crate) id: SongId,
    pub(crate) events: Vec<Timed<ChannelVoiceMessage>>,
    /// If true, this will loop when sent to the synthesizer.
    pub looped: bool,
    /// If true, then this song starts paused.
    pub paused: bool,
}

impl MidiSong {
    /// Get the ID for this song.
    ///
    /// While MidiSong implements [`SongWriter`],
    /// the value of ID is never optional in this case.
    #[inline]
    pub fn id(&self) -> SongId {
        self.id
    }
    /// Returns a builder to build a midi song
    pub fn builder() -> MidiSongBuilder {
        MidiSongBuilder::default()
    }
    /// Create a set of commands
    pub fn new(events: Vec<Timed<ChannelVoiceMessage>>) -> Self {
        Self {
            id: SongId::default(),
            events,
            looped: false,
            paused: false,
        }
    }
    /// Get a mutable reference to the events
    pub fn events_mut(&mut self) -> &mut Vec<Timed<ChannelVoiceMessage>> {
        &mut self.events
    }
    /// Start the song paused
    pub fn set_paused(mut self) -> Self {
        self.paused = true;
        self
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

impl SongWriter for MidiSong {
    fn song_id(&self) -> Option<SongId> {
        Some(self.id)
    }
    fn events(&self) -> impl Iterator<Item = Timed<ChannelVoiceMessage>> {
        self.events.iter().copied()
    }
    fn looped(&self) -> bool {
        self.looped
    }
    fn paused(&self) -> bool {
        self.paused
    }
}
