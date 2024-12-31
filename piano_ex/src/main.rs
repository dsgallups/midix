use bevy::prelude::*;
use bevy_midix::prelude::*;
mod load;
mod piano;
mod synth;
pub mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MidiOutputPlugin)
        .add_plugins((load::plugin, piano::plugin, ui::plugin, synth::plugin))
        .run();
}
