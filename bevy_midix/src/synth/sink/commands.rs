use bevy::prelude::*;
use midix::prelude::ChannelVoiceMessage;

/// A set of commands
pub struct SinkCommands(pub(crate) Vec<SinkCommand>);

impl SinkCommands {
    /// Create a set of commands
    pub fn new(commands: Vec<SinkCommand>) -> Self {
        Self(commands)
    }
}

/// Send a command to the synth to play a note
pub struct SinkCommand {
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
