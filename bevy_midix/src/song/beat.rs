use midix::{Key, prelude::Channel};

use super::MidiSong;

/// A struct to define what goes on within a beat
pub struct Beat<'a> {
    pub(crate) song: &'a mut MidiSong,
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

impl ChannelBeat<'_, '_> {
    /// play a note for this channel. Does not override other notes that will be played.
    pub fn play_note(&mut self, key: Key) -> &mut Self {
        todo!()
    }

    /// play some notes for this channel. Does not override other notes that will be played.
    pub fn play_notes<Keys>(&mut self, keys: Keys) -> &mut Self
    where
        Keys: IntoIterator<Item = Key>,
    {
        todo!()
    }
}
