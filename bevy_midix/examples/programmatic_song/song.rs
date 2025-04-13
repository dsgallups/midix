use bevy::prelude::*;
use bevy_midix::{midix::prelude::*, prelude::*};
pub fn make_simple_song() {
    // new song with 120 beats per minute
    //
    // and 4 beats per measure.
    let mut song = MidiSong::new(120., 4);

    song.channel(Channel::One)
        .set_voice(Program::new(1).unwrap());
    song.channel(Channel::Two)
        .set_voice(Program::new(8).unwrap());

    song.beat(1)
        .channel(Channel::One)
        .play_note(Key::new(Note::C, Octave::new(3)));
    song.beat(1).channel(Channel::Two).play_notes([
        Key::new(Note::E, Octave::new(3)),
        Key::new(Note::G, Octave::new(5)),
    ]);
}
