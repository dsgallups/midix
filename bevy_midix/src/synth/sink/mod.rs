/*

This Sink will send events to another thread that will constantly poll/flush command out to the synth.
*/
use bevy::{prelude::*, tasks::IoTaskPool};
use crossbeam_channel::Sender;
use midix::prelude::ChannelVoiceMessage;

use super::{Synth, SynthState};

mod task;
use task::*;

/// This preprocesses things send to it into sink commands,
/// for sink commands to then handle timing (and to push to the synth)
#[derive(Resource)]
pub struct MidiSink {
    command_channel: Sender<SinkCommand>,
}

/// Send a command to the synth to play a note
struct SinkCommand {
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

///Run once
pub(super) fn setup(mut commands: Commands, synth: Res<Synth>, already_ran: Local<bool>) {
    if *already_ran {
        return;
    }
    let (sender, receiver) = crossbeam_channel::unbounded::<SinkCommand>();

    let SynthState::Loaded(synth_channel) = &synth.synthesizer else {
        return;
    };
    let synth_channel = synth_channel.clone();

    let thread_pool = IoTaskPool::get();
    thread_pool
        .spawn(SinkTask::new(synth_channel, receiver))
        .detach();

    commands.insert_resource(MidiSink {
        command_channel: sender,
    });
}
