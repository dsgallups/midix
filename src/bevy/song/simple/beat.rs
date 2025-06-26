use crate::{
    Key, Velocity,
    prelude::{Channel, ChannelVoiceMessage, VoiceEvent},
};

use super::SimpleMidiSong;

/// A struct to define what goes on within a beat
pub struct Beat<'a> {
    pub(crate) song: &'a mut SimpleMidiSong,
    pub(crate) beat_no: u64,
}

impl<'s> Beat<'s> {
    /// Do something at for a particular channel on this beat
    pub fn channel<'b>(&'b mut self, channel: Channel) -> ChannelBeat<'b, 's> {
        ChannelBeat {
            beat: self,
            channel,
        }
    }
}

/// A struct that will tell a channel to do something at a particular beat
pub struct ChannelBeat<'b, 's> {
    beat: &'b mut Beat<'s>,
    channel: Channel,
}

impl<'b, 's> ChannelBeat<'b, 's> {
    /// play a note for this channel. Does not override other notes that will be played.
    pub fn play(self, key: Key) -> &'b mut Beat<'s> {
        let event = ChannelVoiceMessage::new(self.channel, VoiceEvent::note_on(key, Velocity::MAX));

        self.beat.song.add_event(self.beat.beat_no, event);
        self.beat
    }

    /// play some notes for this channel. Does not override other notes that will be played.
    pub fn play_notes<Keys>(self, keys: Keys) -> &'b mut Beat<'s>
    where
        Keys: IntoIterator<Item = Key>,
    {
        let events = keys.into_iter().map(|key| {
            ChannelVoiceMessage::new(self.channel, VoiceEvent::note_on(key, Velocity::MAX))
        });
        self.beat.song.add_events(self.beat.beat_no, events);
        self.beat
    }
}
