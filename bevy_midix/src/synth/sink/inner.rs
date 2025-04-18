use midix::prelude::ChannelVoiceMessage;

pub struct InnerCommand {
    pub time_to_send: u64,
    pub command: ChannelVoiceMessage,
}
