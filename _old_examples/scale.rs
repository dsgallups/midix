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
        .add_systems(Update, scale_me)
        .run();
}

/// Take a look here for some soundfonts:
///
/// <https://sites.google.com/site/soundfonts4u/>
fn load_sf2(asset_server: Res<AssetServer>, mut synth: ResMut<Synth>) {
    synth.use_soundfont(asset_server.load("soundfont.sf2"));
}

struct Scale {
    timer: Timer,
    current_key: Key,
    note_on: bool,
    forward: bool,
    incremented_by: u8,
    max_increment: u8,
}

impl Scale {
    pub fn calculate_next_key(&mut self) {
        if self.forward {
            if self.incremented_by == self.max_increment {
                self.forward = false;
                self.incremented_by -= 1;
                self.current_key -= 1;
            } else {
                self.incremented_by += 1;
                self.current_key += 1;
            }
        } else if self.incremented_by == 0 {
            self.forward = true;
            self.incremented_by += 1;
            self.current_key += 1;
        } else {
            self.incremented_by -= 1;
            self.current_key -= 1;
        }
    }
}

impl Default for Scale {
    fn default() -> Self {
        let timer = Timer::new(Duration::from_millis(200), TimerMode::Repeating);
        Scale {
            timer,
            current_key: Key::new(Note::C, Octave::new(2)),
            note_on: true,
            forward: true,
            incremented_by: 0,
            max_increment: 11,
        }
    }
}

fn scale_me(synth: Res<Synth>, time: Res<Time>, mut scale: Local<Scale>) {
    if !synth.is_ready() {
        return;
    }
    scale.timer.tick(time.delta());
    if !scale.timer.just_finished() {
        return;
    }
    if scale.note_on {
        info!("Note on {}!", scale.current_key);
        //play note on
        _ = synth.push_event(ChannelVoiceMessage::new(
            Channel::One,
            VoiceEvent::note_on(scale.current_key, Velocity::MAX),
        ));
    } else {
        info!("Note off {}!", scale.current_key);
        _ = synth.push_event(ChannelVoiceMessage::new(
            Channel::One,
            VoiceEvent::note_off(scale.current_key, Velocity::MAX),
        ));
        scale.calculate_next_key()
    }

    scale.note_on = !scale.note_on;
}
