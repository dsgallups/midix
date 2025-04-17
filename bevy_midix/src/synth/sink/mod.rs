/*

This Sink will send events to another thread that will constantly poll/flush command out to the synth.
*/
use bevy::prelude::*;
use crossbeam_channel::Sender;
use midix::prelude::ChannelVoiceMessage;

mod task;
pub use task::*;

use crate::song::SimpleMidiSong;

use super::MidiCommandSource;

/// This preprocesses things send to it into sink commands,
/// for sink commands to then handle timing (and to push to the synth)
#[derive(Resource)]
pub struct MidiSink {
    command_channel: Sender<SinkCommand>,
}
impl MidiSink {
    pub(super) fn new(command_channel: Sender<SinkCommand>) -> Self {
        Self { command_channel }
    }
    ///todo: a trait object
    pub fn push_audio(song: &impl MidiCommandSource) {
        todo!()
    }
}

/// Send a command to the synth to play a note
pub struct SinkCommand {
    timestamp: u64,
    event: ChannelVoiceMessage,
}
impl SinkCommand {
    /// Create a command to play a note to the synth.
    ///
    /// Timestamp is delta micros from now.
    pub fn new(timestamp: u64, event: ChannelVoiceMessage) -> Self {
        Self { timestamp, event }
    }
}
