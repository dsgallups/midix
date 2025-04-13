use std::iter;

use bevy::prelude::*;
use fnv::FnvHashMap;
use midix::prelude::*;

use super::{Beat, ChannelSettings, MidiSong};

/// A builder designed to make simple songs.
///
/// Add a few notes, and then call [`SimpleMidiSong::build`] to get a [`MidiSong`]
///
/// Playing using the beat method, you can play a single tone for a whole beat.
///
/// it will handle the rest.
pub struct SimpleMidiSong {
    beats_per_minute: f64,
    beats_per_measure: u16,

    pub(crate) channel_presets: FnvHashMap<Channel, Program>,

    beats: FnvHashMap<u64, Vec<ChannelVoiceMessage>>,
    last_beat: u64,
}

impl SimpleMidiSong {
    /// Creates a new simple song with a bpm and beats per measure.
    ///
    pub fn new(beats_per_minute: f64, beats_per_measure: u16) -> Self {
        Self {
            beats_per_minute,
            beats_per_measure,
            channel_presets: Default::default(),
            beats: Default::default(),
            last_beat: 0,
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

    /// Converts my events into a playable song!
    pub fn build(mut self) -> MidiSong {
        let mut song = MidiSong::new(self.beats_per_minute);
        let mut next_beat_additions = Vec::new();
        // need to turn off the last beat
        for i in 1..=(self.last_beat + 1) {
            let Some(events) = self.beats.remove(&i) else {
                song.push_beat_events(next_beat_additions.iter().copied());
                next_beat_additions.clear();
                continue;
            };

            let additions_for_this_beat = next_beat_additions.clone();
            next_beat_additions.clear();

            for event in events.iter() {
                // add off events for the next beat
                if event.is_note_on() {
                    next_beat_additions.push(ChannelVoiceMessage::new(
                        event.channel(),
                        VoiceEvent::note_off(*event.key().unwrap(), Velocity::max()),
                    ));
                }
            }
            song.push_beat_events(additions_for_this_beat.into_iter().chain(events));
        }
        song
    }
}

#[test]
fn make_simple_song() {
    use pretty_assertions::assert_eq;
    let mut simple_song = SimpleMidiSong::new(120., 4);
    simple_song
        .beat(1)
        .channel(Channel::One)
        .play_note(Key::new(Note::A, Octave::new(2)));
    simple_song
        .beat(2)
        .channel(Channel::One)
        .play_note(Key::new(Note::B, Octave::new(4)));

    simple_song
        .beat(5)
        .channel(Channel::One)
        .play_note(Key::new(Note::F, Octave::new(1)));

    let song = simple_song.build();

    let queue = song.queue;
    assert_eq!(
        queue[0],
        vec![ChannelVoiceMessage::new(
            Channel::One,
            VoiceEvent::note_on(Key::new(Note::A, Octave::new(2)), Velocity::max())
        )]
    );
    assert_eq!(
        queue[1],
        vec![
            ChannelVoiceMessage::new(
                Channel::One,
                VoiceEvent::note_off(Key::new(Note::A, Octave::new(2)), Velocity::max())
            ),
            ChannelVoiceMessage::new(
                Channel::One,
                VoiceEvent::note_on(Key::new(Note::B, Octave::new(4)), Velocity::max())
            )
        ]
    );
    assert_eq!(
        queue[2],
        vec![ChannelVoiceMessage::new(
            Channel::One,
            VoiceEvent::note_off(Key::new(Note::B, Octave::new(4)), Velocity::max())
        ),]
    );
    assert_eq!(queue[3], vec![]);
    assert_eq!(
        queue[4],
        vec![ChannelVoiceMessage::new(
            Channel::One,
            VoiceEvent::note_on(Key::new(Note::F, Octave::new(1)), Velocity::max())
        )]
    );
    // notice that there's a sixth measure here
    assert_eq!(
        queue[5],
        vec![ChannelVoiceMessage::new(
            Channel::One,
            VoiceEvent::note_off(Key::new(Note::F, Octave::new(1)), Velocity::max())
        )]
    );
}
