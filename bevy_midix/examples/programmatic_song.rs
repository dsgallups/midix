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
    let mut s = SimpleMidiSong::new(140. * 2.);

    use Channel::*;

    s.channel(One).set_voice(Program::new(31).unwrap());
    s.channel(Two).set_voice(Program::new(8).unwrap());

    //s.channel(One)
    let mut section_repeat = SimpleSection::default();
    section_repeat
        .beat(1)
        .play(key!(F, 3))
        .beat(2)
        .play(key!(C, 3))
        .beat(3)
        .play(key!(F, 3))
        .beat(4)
        .play(key!(G, 3))
        .beat(5)
        .play(key!(GSharp, 3))
        .beat(6)
        .play(key!(DSharp, 3))
        .beat(7)
        .play(key!(GSharp, 3))
        .beat(8)
        .play(key!(DSharp, 3))
        .beat(9)
        .play(key!(CSharp, 3))
        .beat(10)
        .play(key!(F, 3))
        .beat(11)
        .play(key!(CSharp, 4))
        .beat(12)
        .play(key!(GSharp, 3))
        .beat(13)
        .play(key!(C, 3))
        .beat(14)
        .play(key!(E, 3))
        .beat(15)
        .play(key!(C, 4))
        .beat(16)
        .play(key!(G, 3));
    // we'll play this section twice
    s.channel(One)
        .play_section(&section_repeat, 0)
        .play_section(&section_repeat, 16);

    for (i, base_key) in [
        key!(F, 2),
        key!(G, 2),
        key!(GSharp, 2),
        key!(ASharp, 2),
        key!(C, 3),
        key!(ASharp, 2),
        key!(GSharp, 2),
        key!(G, 2),
        key!(CSharp, 2),
    ]
    .into_iter()
    .enumerate()
    {
        let higher_key = Key::new(base_key.note(), base_key.octave() + 2);
        s.channel(Two)
            .beat(i as u64 + 17)
            .play_notes([base_key, higher_key]);
    }

    // rest

    for (i, base_key) in [key!(CSharp, 2), key!(DSharp, 2), key!(C, 2)]
        .into_iter()
        .enumerate()
    {
        let higher_key = Key::new(base_key.note(), base_key.octave() + 2);
        s.channel(Two)
            .beat(i as u64 + 27)
            .play_notes([base_key, higher_key]);
    }

    // a MidiSong, ready to go!
    let song = s.build();

    commands.insert_resource(song);
}
