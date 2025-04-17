use std::{pin::Pin, task::Context};

/*

This Sink will send events to another thread that will constantly poll/flush command out to the synth.
*/
use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};
use midix::prelude::ChannelVoiceMessage;

use crate::song::SimpleMidiSong;

use super::MidiCommandSource;

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

/// This struct is essentially the glue
/// that determines when to send messages to the synthesizer.
///
/// It needs its own thread because it's going to need to update its timer
///
/// as frequently as possible.
pub(crate) struct SinkTask {
    synth_channel: Sender<ChannelVoiceMessage>,
    commands: Receiver<SinkCommand>,
}

impl SinkTask {
    pub fn new(
        synth_channel: Sender<ChannelVoiceMessage>,
        commands: Receiver<SinkCommand>,
    ) -> Self {
        Self {
            synth_channel,
            commands,
        }
    }
}

impl Future for SinkTask {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> std::task::Poll<Self::Output> {
        todo!()
    }
}
