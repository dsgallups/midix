use bevy::asset::uuid::Uuid;
use midix::prelude::ChannelVoiceMessage;

pub struct InnerCommand {
    pub time_to_send: u64,
    pub parent: Uuid,
    pub command: ChannelVoiceMessage,
}
