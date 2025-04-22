use midix::prelude::ChannelVoiceMessage;

use super::SongId;

pub struct InnerCommand {
    pub time_to_send: u64,
    pub parent: Option<SongId>,
    pub command: ChannelVoiceMessage,
}
