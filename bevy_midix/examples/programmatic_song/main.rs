use std::time::Duration;

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
        .add_systems(Startup, load_sf2)
        .add_systems(Update, make_song)
        .run();
}

/// Take a look here for some soundfonts:
///
/// <https://sites.google.com/site/soundfonts4u/>
fn load_sf2(asset_server: Res<AssetServer>, mut synth: ResMut<Synth>) {
    synth.use_soundfont(asset_server.load("8bitsf.sf2"));
}

struct VoiceChanger {
    timer: Timer,
    voice_number: u8,
}

impl Default for VoiceChanger {
    fn default() -> Self {
        let timer = Timer::new(Duration::from_millis(1000), TimerMode::Repeating);
        VoiceChanger {
            timer,
            voice_number: 0,
        }
    }
}

fn make_song(synth: Res<Synth>, time: Res<Time>, mut scale: Local<VoiceChanger>) {
    if !synth.is_ready() {
        return;
    }
    scale.timer.tick(time.delta());
    if !scale.timer.just_finished() {
        return;
    }
    const BASE_OCTAVE: i8 = 4;
    const C_CHORD: [Key; 1] = [Key::new(Note::C, Octave::new(BASE_OCTAVE))];
    info!("Voice {}!", scale.voice_number);
    for key in C_CHORD {
        synth.handle_event(ChannelVoiceMessage::new(
            Channel::One,
            VoiceEvent::note_off(key, Velocity::max()),
        ));
    }
    if scale.voice_number == 127 {
        scale.voice_number = 0
    } else {
        scale.voice_number += 1;
    }
    synth.handle_event(ChannelVoiceMessage::new(
        Channel::One,
        // unwrapping is okay, because we don't go past 127.
        VoiceEvent::program_change(Program::new(scale.voice_number).unwrap()),
    ));
    for key in C_CHORD {
        synth.handle_event(ChannelVoiceMessage::new(
            Channel::One,
            VoiceEvent::note_on(key, Velocity::max()),
        ));
    }
}
