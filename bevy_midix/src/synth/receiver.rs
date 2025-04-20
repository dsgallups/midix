use bevy::prelude::*;
use crossbeam_channel::Receiver;
use midix::prelude::ChannelVoiceMessage;

/// This is a reader that will recieve commands sent to the synth.
#[derive(Resource, Component, Clone)]
pub(super) struct SynthCommandReaderReceiver {
    pub(super) receiver: Receiver<ChannelVoiceMessage>,
}

/// connects the channel with the resource
pub(super) fn poll_receiver(
    mut ev: EventWriter<ChannelVoiceMessage>,
    command_receiver: Res<SynthCommandReaderReceiver>,
) {
    ev.write_batch(command_receiver.receiver.try_iter());
}
