mod channel;
pub use channel::*;

use midix::prelude::{Channel, ChannelVoiceMessage, Timed};

use super::MidiSong;

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
    /// Add many timed channel voice messages
    pub fn add_many(
        &mut self,
        events: impl IntoIterator<Item = Timed<ChannelVoiceMessage>>,
    ) -> &mut Self {
        self.events.extend(events);
        self
    }

    /// Build a midi song from the provided events
    pub fn build(self) -> MidiSong {
        MidiSong::new(self.events)
    }
}
