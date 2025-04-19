use midix::prelude::ChannelVoiceMessage;

use super::SongId;

pub struct InnerCommand {
    pub time_to_send: u64,
    pub parent: SongId,
    pub command: ChannelVoiceMessage,
}
