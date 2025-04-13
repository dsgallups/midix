use bevy::prelude::*;
use fnv::FnvHashMap;
use midix::prelude::*;

mod channel_settings;
pub use channel_settings::*;

mod beat;
pub use beat::*;

/// A component designed to make simple songs.
///
/// Playing using the beat method, you can play a single tone for a whole beat.
///
/// it will handle the rest.
#[derive(Component, Resource)]
pub struct MidiSong {
    beats_per_minute: f64,
    beats_per_measure: u16,

    channel_presets: FnvHashMap<Channel, Program>,

    beats: FnvHashMap<u64, Vec<ChannelVoiceMessage>>,
}

impl MidiSong {
    /// Creates a new simple song with a bpm and beats per measure.
    ///
    pub fn new(beats_per_minute: f64, beats_per_measure: u16) -> Self {
        Self {
            beats_per_minute,
            beats_per_measure,
            channel_presets: Default::default(),
            beats: Default::default(),
        }
    }

    /// Set values for a channel
    pub fn channel(&mut self, channel: Channel) -> ChannelSettings<'_> {
        ChannelSettings {
            song: self,
            channel,
        }
    }

    /// Do something on beat. Beats start at 1.
    pub fn beat(&mut self, beat_no: u64) -> Beat<'_> {
        Beat {
            song: self,
            beat_no,
        }
    }

    /// Add an event
    pub fn add_event(&mut self, beat_no: u64, event: ChannelVoiceMessage) {
        let note_on = event.is_note_on();
        // here, we will add a note off for the next beat.
        let current_beat = self.beats.entry(beat_no).or_default();
        current_beat.push(event);

        if note_on {
            let next_beat = self.beats.entry(beat_no + 1).or_default();
            next_beat.push(ChannelVoiceMessage::new(
                event.channel(),
                VoiceEvent::note_off(*event.key().unwrap(), Velocity::max()),
            ));
        }
    }

    /// Add a set of events toa beat.
    pub fn add_events<Msgs>(&mut self, beat_no: u64, events: Msgs)
    where
        Msgs: IntoIterator<Item = ChannelVoiceMessage>,
    {
        for event in events {
            self.add_event(beat_no, event);
        }
    }
}
