use std::time::Duration;

use bevy::prelude::*;
use midix::prelude::*;

mod channel_settings;
pub use channel_settings::*;

mod beat;
pub use beat::*;

mod simple_song;
pub use simple_song::*;

/// A component designed to make simple songs.
///
/// Playing using the beat method, you can play a single tone for a whole beat.
///
/// it will handle the rest.
///
/// The methods for creating a [`MidiSong`] are not yet public.
///
/// Ideally, a MidiSong should be able to handle all possible notes at any time,
/// and that's not yet the case.
#[derive(Component, Resource)]
pub struct MidiSong {
    timer: Timer,

    current_beat: u16,

    // each indice is a beat
    queue: Vec<Vec<ChannelVoiceMessage>>,
}

impl MidiSong {
    /// Set the beats per minute for the song
    pub(crate) fn new(beats_per_minute: f64) -> Self {
        let micros_per_beat = 60_000_000. / beats_per_minute;

        let timer = Timer::new(
            Duration::from_micros(micros_per_beat.round() as u64),
            TimerMode::Repeating,
        );

        Self {
            timer,
            current_beat: 0,
            queue: Vec::new(),
        }
        // the timer will tick every beat
    }
    /// push events for this beat
    pub(crate) fn push_beat_events<I>(&mut self, events: I) -> &mut Self
    where
        I: Iterator<Item = ChannelVoiceMessage>,
    {
        self.queue.push(events.collect());
        self
    }
}
