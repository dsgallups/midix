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

    current_beat: usize,

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
    }
    /// push events for this beat
    pub(crate) fn push_beat_events<I>(&mut self, events: I) -> &mut Self
    where
        I: Iterator<Item = ChannelVoiceMessage>,
    {
        self.queue.push(events.collect());
        self
    }
    /// Get the current beat. 0 means the song is at the beginning, waiting to play.
    pub fn current_beat(&self) -> usize {
        self.current_beat
    }
    /// reset the beat to zero.
    pub fn restart(&mut self) {
        self.current_beat = 0;
        self.timer.reset();
    }
    /// Set the current beat number.
    pub fn skip_to(&mut self, beat: usize) {
        self.current_beat = beat.saturating_sub(1);
        self.timer.reset();
    }

    /// returns true if the current beat is past the end of the song.
    pub fn finished(&self) -> bool {
        self.current_beat >= self.queue.len()
    }
    /// Pass a [`Timer::delta()`] to get events to play right now.
    pub fn get_events(&mut self, delta: Duration) -> Option<&[ChannelVoiceMessage]> {
        self.timer.tick(delta);
        if self.timer.just_finished() {
            let res = self.queue.get(self.current_beat).map(|v| v.as_slice());
            self.current_beat += 1;
            return res;
        }
        None
    }
}
