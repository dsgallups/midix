mod channel;
use std::vec::Vec;

pub use channel::*;

use crate::prelude::{Channel, ChannelVoiceMessage, Timed};

use super::MidiSong;

/// A struct to build a [`MidiSong`] programatically.
///
/// This struct does not consider beats per minute or anything of that sort.
///
/// Straight micros, delta seconds.
#[derive(Default, Clone, Debug)]
pub struct MidiSongBuilder {
    accumulated_time: u64,
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

    /// Add a timed channel voice message. event should be in DELTA micros.
    pub fn add(&mut self, mut event: Timed<ChannelVoiceMessage>) -> &mut Self {
        self.accumulated_time += event.timestamp;
        event.timestamp = self.accumulated_time;

        self.events.push(event);
        self
    }
    /// Add many timed channel voice messages
    pub fn add_many(
        &mut self,
        events: impl IntoIterator<Item = Timed<ChannelVoiceMessage>>,
    ) -> &mut Self {
        self.events.extend(events.into_iter().map(|mut event| {
            self.accumulated_time += event.timestamp;
            event.timestamp = self.accumulated_time;
            event
        }));
        self
    }

    /// Build a midi song from the provided events
    pub fn build(self) -> MidiSong {
        MidiSong::new(self.events)
    }
}
