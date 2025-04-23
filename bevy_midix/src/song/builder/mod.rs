mod channel;
pub use channel::*;

use midix::prelude::{Channel, ChannelVoiceMessage, Timed};

/// A struct to build a [`MidiSong`] programatically.
///
/// This struct does not consider beats per minute or anything of that sort.
///
/// Straight micros.
#[derive(Default, Clone, Debug)]
pub struct MidiSongBuilder {
    events: Vec<Timed<ChannelVoiceMessage>>,
}

impl MidiSongBuilder {
    /// Send a lot of events to one channel efficiently
    pub fn channel(&mut self, channel: Channel) -> ChannelBuilder<'_> {
        ChannelBuilder {
            builder: self,
            channel,
        }
    }

    /// Add a timed channel voice message.
    pub fn add(&mut self, event: Timed<ChannelVoiceMessage>) -> &mut Self {
        self.events.push(event);
        self
    }
}
