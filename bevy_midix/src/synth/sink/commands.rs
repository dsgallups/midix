use midix::prelude::*;

use crate::song::SongId;

/// Command the sink to do something
pub(crate) enum SinkCommand {
    /// Play an event in time x
    PlayEvent(Timed<ChannelVoiceMessage>),
    /// Play a new song
    NewSong {
        id: SongId,
        looped: bool,
        /// The associated events with the song
        commands: Vec<Timed<ChannelVoiceMessage>>,
    },
    /// Stop a song
    Stop {
        song_id: Option<SongId>,
        stop_voices: bool,
    },
}

pub(crate) struct InnerCommand {
    pub(crate) time_to_send: u64,
    pub(crate) parent: Option<SongId>,
    pub(crate) command: ChannelVoiceMessage,
}
