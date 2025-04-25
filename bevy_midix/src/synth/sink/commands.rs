use midix::prelude::*;

use crate::song::SongId;

/// The type of song that will be sent to the synth
#[derive(Clone, Copy)]
pub(crate) enum SongType {
    /// No identifier, and therefore, no looping
    Anonymous,
    /// An identifier, and therefore, looping
    Identified {
        /// The song identifer
        id: SongId,
        /// true if it loops
        looped: bool,
    },
}

impl SongType {
    pub(crate) fn id(&self) -> Option<SongId> {
        match self {
            SongType::Anonymous => None,
            SongType::Identified { id, .. } => Some(*id),
        }
    }
}

/// Command the sink to do something
pub(crate) enum SinkCommand {
    /// Play a new song
    NewSong {
        /// What kind of song is this?
        song_type: SongType,
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
