use bevy::asset::uuid::Uuid;
use midix::prelude::ChannelVoiceMessage;

/// The identifier of a certain midi song
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct SongId(Uuid);

impl Default for SongId {
    fn default() -> Self {
        SongId(Uuid::new_v4())
    }
}

/// A set of commands
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
    pub fn looped(mut self) -> Self {
        self.looped = true;
        self
    }
    /// The ID of the commands to do something later
    pub fn id(&self) -> SongId {
        self.id
    }
}

/// Send a command to the synth to play a note
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
}
