use std::time::Duration;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};

use midix::prelude::*;

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
        .add_systems(Startup, load_sf2)
        .add_systems(Update, make_song)
        .run();
}

/// Take a look here for some soundfonts:
///
/// <https://sites.google.com/site/soundfonts4u/>
fn load_sf2(asset_server: Res<AssetServer>, mut synth: ResMut<Synth>) {
    synth.use_soundfont(asset_server.load("soundfont.sf2"));
}

struct PlaySongAgain {
    timer: Timer,
    count: u8,
}
impl Default for PlaySongAgain {
    fn default() -> Self {
        let timer = Timer::new(Duration::from_millis(5000), TimerMode::Repeating);
        Self { timer, count: 0 }
    }
}
fn make_song(
    mut synth: ResMut<Synth>,
    mut play_again: Local<Option<PlaySongAgain>>,
    timer: Res<Time>,
) {
    if !synth.is_ready() {
        return;
    }
    let count = if let Some(ref mut play_again) = *play_again {
        play_again.timer.tick(timer.delta());
        if !play_again.timer.just_finished() {
            return;
        }
        play_again.count += 1;
        play_again.count
    } else {
        *play_again = Some(PlaySongAgain::default());
        0
    };

    let mut song = SimpleMidiSong::new(400.);

    use Channel::*;

    println!("voice: {count}");
    song.channel(One)
        .set_voice(Program::new(count).unwrap())
        .set_volume(Velocity::new(80).unwrap());
    song.channel(Two).set_voice(Program::new(count).unwrap());

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
    song.channel(One)
        .play_section(&section_repeat, 0)
        .play_section(&section_repeat, 16);

    for i in 17..=24 {
        song.channel(Two).beat(i).play(key!(F, 1));
    }
    for i in 25..=28 {
        song.channel(Two).beat(i).play(key!(CSharp, 1));
    }
    for i in 29..=32 {
        song.channel(Two).beat(i).play(key!(C, 1));
    }

    // for (i, base_key) in [
    //     key!(F, 2),
    //     key!(G, 2),
    //     key!(GSharp, 2),
    //     key!(ASharp, 2),
    //     key!(C, 3),
    //     key!(ASharp, 2),
    //     key!(GSharp, 2),
    //     key!(G, 2),
    //     key!(CSharp, 2),
    // ]
    // .into_iter()
    // .enumerate()
    // {
    //     let higher_key = Key::new(base_key.note(), base_key.octave() + 2);
    //     s.channel(Two)
    //         .beat(i as u64 + 17)
    //         .play_notes([base_key, higher_key]);
    // }

    // // rest

    // for (i, base_key) in [key!(CSharp, 2), key!(DSharp, 2), key!(C, 2)]
    //     .into_iter()
    //     .enumerate()
    // {
    //     let higher_key = Key::new(base_key.note(), base_key.octave() + 2);
    //     s.channel(Two)
    //         .beat(i as u64 + 27)
    //         .play_notes([base_key, higher_key]);
    // }

    synth.push_audio(song.into_song()).unwrap();
}
