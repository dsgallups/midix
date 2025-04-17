use std::{pin::Pin, task::Context};

use crossbeam_channel::{Receiver, Sender};
use midix::prelude::ChannelVoiceMessage;

use super::SinkCommand;

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
