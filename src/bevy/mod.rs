#![doc = r#"
Bevy plugin that uses [`midix`](https://crates.io/crates/midix),
[`midir`](https://github.com/Boddlnagg/midir), and a [`rustysynth`](https://github.com/sinshu/rustysynth) fork to play midi sounds!

Read from MIDI devices, MIDI files, and programmable input, and output to user audio with a soundfont!

## Features
- Enable `web` for WASM compatibility

## Example
```rust, no_run
use std::time::Duration;
use bevy_platform::prelude::*;use bevy::{
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
    // don't do anything until the soundfont has been loaded
    if !synth.is_ready() {
        return;
    }
    scale.timer.tick(time.delta());
    if !scale.timer.just_finished() {
        return;
    }
    if scale.note_on {
        //play note on
        _ = synth.push_event(ChannelVoiceMessage::new(
            Channel::One,
            VoiceEvent::note_on(scale.current_key, Velocity::MAX),
        ));
    } else {
        //turn off the note
        _ = synth.push_event(ChannelVoiceMessage::new(
            Channel::One,
            VoiceEvent::note_off(scale.current_key, Velocity::MAX),
        ));
        scale.calculate_next_key()
    }

    scale.note_on = !scale.note_on;
}
```
"#]
#![warn(missing_docs)]

#[cfg(feature = "std")]
pub mod asset;
pub mod input;
pub mod output;
pub mod plugin;
pub mod settings;
pub mod synth;

pub mod song;

/// Commonly re-exported types
pub mod prelude {
    pub use crate::bevy::{input::*, output::*, plugin::*, settings::*, song::*, synth::*};

    #[cfg(feature = "std")]
    pub use crate::bevy::asset::*;
}
