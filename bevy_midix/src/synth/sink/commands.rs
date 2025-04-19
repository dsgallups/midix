use bevy::asset::uuid::Uuid;
use midix::prelude::ChannelVoiceMessage;

/// A set of commands
pub struct MidiSong {
    pub(crate) id: Uuid,
    pub(crate) commands: Vec<SinkCommand>,
    pub(crate) looped: bool,
}

impl MidiSong {
    /// Create a set of commands
    pub fn new(commands: Vec<SinkCommand>) -> Self {
        Self {
            id: Uuid::new_v4(),
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
    pub fn id(&self) -> Uuid {
        self.id
    }
}

/// Send a command to the synth to play a note
pub struct SinkCommand {
    /// Micros
    pub(crate) timestamp: u64,
    pub(crate) event: ChannelVoiceMessage,
}
impl SinkCommand {
    /// Create a command to play a note to the synth.
    ///
    /// Timestamp is delta micros from now.
    pub fn new(timestamp: u64, event: ChannelVoiceMessage) -> Self {
        Self { timestamp, event }
    }
}
