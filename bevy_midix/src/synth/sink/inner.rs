use midix::prelude::ChannelVoiceMessage;

use crate::song::SongId;

pub struct InnerCommand {
    pub time_to_send: u64,
    pub parent: Option<SongId>,
    pub command: ChannelVoiceMessage,
}
