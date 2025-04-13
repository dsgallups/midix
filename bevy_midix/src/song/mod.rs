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
#[derive(Component)]
pub struct MidiSong {
    beats_per_minute: f64,
    beats_per_measure: u16,

    channel_presets: FnvHashMap<Channel, Program>,
}

impl MidiSong {
    /// Creates a new simple song with a bpm and beats per measure.
    ///
    pub fn new(beats_per_minute: f64, beats_per_measure: u16) -> Self {
        Self {
            beats_per_minute,
            beats_per_measure,
            channel_presets: Default::default(),
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
}
