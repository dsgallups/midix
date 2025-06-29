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
        .add_systems(Update, iterate_voices)
        .run();
}

/// Take a look here for some soundfonts:
///
/// <https://sites.google.com/site/soundfonts4u/>
fn load_sf2(asset_server: Res<AssetServer>, mut synth: ResMut<Synth>) {
    synth.use_soundfont(asset_server.load("soundfont.sf2"));
}

struct VoiceChanger {
    timer: Timer,
    count: u16,
}

impl Default for VoiceChanger {
    fn default() -> Self {
        let timer = Timer::new(Duration::from_millis(500), TimerMode::Repeating);
        VoiceChanger { timer, count: 0 }
    }
}

fn iterate_voices(synth: Res<Synth>, time: Res<Time>, mut scale: Local<VoiceChanger>) {
    if !synth.is_ready() {
        return;
    }
    scale.timer.tick(time.delta());

    let lsb = (scale.count & 0x00FF) as u8;
    let msb = (scale.count >> 8) as u8;

    println!("count: {}, {:02x} {:02x}", scale.count, msb, lsb);

    let message = ChannelVoiceMessage::new(
        Channel::One,
        VoiceEvent::PitchBend(PitchBend::new(0, msb).unwrap()),
    );

    println!(
        "count: {}, {:02x} {:02x}. {}, {}",
        scale.count,
        msb,
        lsb,
        message.data_1_byte(),
        message.data_2_byte().unwrap()
    );
    _ = synth.push_event(ChannelVoiceMessage::new(
        Channel::One,
        VoiceEvent::PitchBend(PitchBend::new(0, msb).unwrap()),
    ));
    scale.count += 10;
    if !scale.timer.just_finished() {
        return;
    }
    const BASE_OCTAVE: i8 = 4;
    let key = Key::new(Note::C, Octave::new(BASE_OCTAVE));

    _ = synth.push_event(ChannelVoiceMessage::new(
        Channel::One,
        VoiceEvent::note_on(key, Velocity::MAX),
    ));
}
