use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};

use bevy_midix::{midix::prelude::*, prelude::*};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: Level::INFO,
                ..default()
            }),
            MidiPlugin {
                input: None,
                ..Default::default()
            },
        ))
        .add_systems(Startup, (load_sf2, make_song))
        .add_systems(Update, play_song)
        .run();
}

/// Take a look here for some soundfonts:
///
/// <https://sites.google.com/site/soundfonts4u/>
fn load_sf2(asset_server: Res<AssetServer>, mut synth: ResMut<Synth>) {
    synth.use_soundfont(asset_server.load("soundfont.sf2"));
}

fn play_song(synth: Res<Synth>, time: Res<Time>, mut song: ResMut<MidiSong>) {
    if !synth.is_ready() {
        return;
    }
    let beat = song.current_beat();
    if song.finished() {
        song.restart();
    }
    let Some(events) = song.get_events(time.delta()) else {
        return;
    };
    info!("song beat no: {}", beat + 1);

    for event in events {
        synth.handle_event(*event);
    }
}

pub fn make_song(mut commands: Commands) {
    // <https://musiclab.chromeexperiments.com/Song-Maker/song/5716146745114624>
    //
    // new song with 120 beats per minute
    let mut song_builder = SimpleMidiSong::new(140.);

    let key = key!(C, 2);
    song_builder
        .channel(Channel::One)
        .set_voice(Program::new(1).unwrap());
    song_builder
        .channel(Channel::Two)
        .set_voice(Program::new(8).unwrap());

    song_builder
        .beat(1)
        .channel(Channel::One)
        .play_note(Key::new(Note::C, Octave::new(3)));

    song_builder.beat(1).channel(Channel::Two).play_notes([
        Key::new(Note::E, Octave::new(3)),
        Key::new(Note::G, Octave::new(5)),
    ]);

    // a MidiSong, ready to go!
    let song = song_builder.build();

    commands.insert_resource(song);
}
