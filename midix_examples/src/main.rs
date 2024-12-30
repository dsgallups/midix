use bevy::prelude::*;
mod load;
mod piano;
pub mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((load::plugin, piano::plugin, ui::plugin))
        .run();
}
