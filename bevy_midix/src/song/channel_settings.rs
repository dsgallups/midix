use midix::{Program, prelude::Channel};

use super::SimpleMidiSong;

/// A struct provided to update the settings of a particular channel for a song
pub struct ChannelSettings<'a> {
    pub(crate) song: &'a mut SimpleMidiSong,
    pub(crate) channel: Channel,
}

impl ChannelSettings<'_> {
    /// Set the voice for a channel
    pub fn set_voice(&mut self, program: Program) -> &mut Self {
        self.song.channel_presets.insert(self.channel, program);
        self
    }
}
