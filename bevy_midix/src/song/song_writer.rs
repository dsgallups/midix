use std::iter;

use fnv::FnvHashMap;
use midix::prelude::{Channel, ChannelVoiceMessage, Timed, VoiceEvent};

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
    /// Helper method that will divide events into individual channels
    fn divide_events_into_channels(&self) -> FnvHashMap<Channel, Vec<Timed<VoiceEvent>>> {
        let mut map: FnvHashMap<Channel, Vec<Timed<VoiceEvent>>> = FnvHashMap::default();

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
