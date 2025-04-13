#![doc = r#"
Components to make songs programatically
"#]
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use bevy::prelude::*;
use midix::prelude::*;

mod channel_settings;
pub use channel_settings::*;

mod beat;
pub use beat::*;

mod simple_song;
pub use simple_song::*;

mod section;
pub use section::*;

mod bad_idea;
pub use bad_idea::*;

mod sink;
pub use sink::*;

use crate::synth::Synth;

fn spawn_song_thread(new_song: ResMut<MidiSong>, synth: Option<Res<Synth>>) {
    //todo: spawna  thread to tick event
}
