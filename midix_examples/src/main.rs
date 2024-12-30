use bevy::prelude::*;
use bevy_midix::prelude::*;
mod load;
mod piano;
pub mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MidiOutputPlugin)
        .add_plugins((load::plugin, piano::plugin, ui::plugin))
        .run();
}
