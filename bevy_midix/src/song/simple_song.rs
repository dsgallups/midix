use bevy::prelude::*;
use fnv::FnvHashMap;
use midix::prelude::*;

use crate::synth::{MidiCommandSource, MidiSong, TimedMidiEvent};

use super::{Beat, ChannelModifier};

/// Presets for a channel for a simple song
#[derive(Copy, Clone, Debug)]
pub struct ChannelSettings {
    /// program to use
    pub program: Program,
    /// how loud is it
    pub velocity: Velocity,
}
impl Default for ChannelSettings {
    fn default() -> Self {
        Self {
            program: Program::new(1).unwrap(),
            velocity: Velocity::max(),
        }
    }
}

/// A builder designed to make simple songs.
///
/// Add a few notes, and then call [`SimpleMidiSong::build`] to get a [`MidiSong`]
///
/// Playing using the beat method, you can play a single tone for a whole beat.
///
/// it will handle the rest.
pub struct SimpleMidiSong {
    beats_per_minute: f64,

    pub(crate) channel_presets: FnvHashMap<Channel, ChannelSettings>,

    beats: FnvHashMap<u64, Vec<ChannelVoiceMessage>>,
    last_beat: u64,
}

impl SimpleMidiSong {
    /// Creates a new simple song with a bpm.
    ///
    /// Beats per measure isn't needed. Simple song!
    pub fn new(beats_per_minute: f64) -> Self {
        Self {
            beats_per_minute,
            channel_presets: Default::default(),
            beats: Default::default(),
            last_beat: 0,
        }
    }

    /// Set values for a channel
    pub fn channel(&mut self, channel: Channel) -> ChannelModifier<'_> {
        ChannelModifier {
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
        if beat_no > self.last_beat {
            self.last_beat = beat_no
        }
        let current_beat = self.beats.entry(beat_no).or_default();
        current_beat.push(event);
    }

    /// Add a set of events toa beat.
    pub fn add_events<Msgs>(&mut self, beat_no: u64, events: Msgs)
    where
        Msgs: IntoIterator<Item = ChannelVoiceMessage>,
    {
        if beat_no > self.last_beat {
            self.last_beat = beat_no
        }
        let current_beat = self.beats.entry(beat_no).or_default();
        current_beat.extend(events);
    }
}

impl MidiCommandSource for SimpleMidiSong {
    fn to_commands(&self) -> MidiSong {
        let micros_per_beat = 60_000_000. / self.beats_per_minute;

        let mut next_beat_additions = Vec::new();

        // we'll add the program change for any voices set to next_beat additions
        if !self.channel_presets.is_empty() {
            for (channel, settings) in self.channel_presets.iter() {
                next_beat_additions.push(ChannelVoiceMessage::new(
                    *channel,
                    VoiceEvent::program_change(settings.program),
                ));
            }
        }

        let mut commands = Vec::with_capacity(self.beats.len() * 6);

        for i in 0..=self.last_beat {
            let beat_no = i + 1;
            let timestamp = i * micros_per_beat as u64;
            let Some(events) = self.beats.get(&beat_no) else {
                let iter = next_beat_additions
                    .iter()
                    .copied()
                    .map(|nb| TimedMidiEvent::new(timestamp, nb))
                    .collect::<Vec<_>>();

                next_beat_additions.clear();
                commands.extend(iter);
                continue;
            };

            let additions_for_this_beat = next_beat_additions.clone();
            next_beat_additions.clear();

            for event in events.iter() {
                // add off events for the next beat
                if event.is_note_on() {
                    let channel = event.channel();

                    next_beat_additions.push(ChannelVoiceMessage::new(
                        channel,
                        VoiceEvent::note_off(*event.key().unwrap(), Velocity::max()),
                    ));
                }
            }
            commands.extend(
                additions_for_this_beat
                    .into_iter()
                    .chain(events.iter().copied())
                    .map(|msg| TimedMidiEvent::new(timestamp, msg)),
            );
        }

        MidiSong::new(commands)
    }
}
