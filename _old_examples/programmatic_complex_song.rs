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
fn make_song(synth: Res<Synth>, mut play_again: Local<Option<PlaySongAgain>>, timer: Res<Time>) {
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

    let mut song = MidiSong::builder();

    use Channel::*;

    println!("voice: {count}");
    song.channel(One)
        .program_change(0, Program::new(count).unwrap())
        .note_on(0, key!(C, 4), Velocity::new_unchecked(127))
        .after_touch(0, key!(C, 4), Velocity::new_unchecked(127));
    song.channel(Two)
        .program_change(0, Program::new(count).unwrap());
}
