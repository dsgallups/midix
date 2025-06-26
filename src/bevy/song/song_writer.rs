use core::{hash::BuildHasher, iter};

use crate::prelude::{Channel, ChannelVoiceMessage, Timed, VoiceEvent};
use bevy_platform::{collections::HashMap, prelude::*};

use super::SongId;

/// This defines a song, a file, or otherwise
/// that has timestamps associated with midi events.
///
/// this is named as such not to conflict with [`midix::MidiSource`]
pub trait SongWriter {
    /// Create sink commands this type.
    fn song_id(&self) -> Option<SongId> {
        None
    }
    /// A list of timed events relevant to this song
    fn events(&self) -> impl Iterator<Item = Timed<ChannelVoiceMessage>>;
    /// is this song looped?
    fn looped(&self) -> bool {
        false
    }
    /// Is this song paused, or does it play instantly?
    fn paused(&self) -> bool {
        false
    }
    /// Helper method that will divide events into individual channels using a hasher of the caller's choice.
    fn divide_events_into_channels<S: BuildHasher + Default>(
        &self,
    ) -> HashMap<Channel, Vec<Timed<VoiceEvent>>, S> {
        let mut map: HashMap<Channel, Vec<Timed<VoiceEvent>>, S> = HashMap::default();

        for event in self.events() {
            let voice_events = map.entry(event.event.channel()).or_default();
            voice_events.push(Timed::new(event.timestamp, *event.event.event()));
        }

        map
    }
}

impl SongWriter for Timed<ChannelVoiceMessage> {
    fn events(&self) -> impl Iterator<Item = Timed<ChannelVoiceMessage>> {
        iter::once(*self)
    }
}
impl SongWriter for Vec<Timed<ChannelVoiceMessage>> {
    fn events(&self) -> impl Iterator<Item = Timed<ChannelVoiceMessage>> {
        self.iter().copied()
    }
}
impl SongWriter for [Timed<ChannelVoiceMessage>] {
    fn events(&self) -> impl Iterator<Item = Timed<ChannelVoiceMessage>> {
        self.iter().copied()
    }
}
