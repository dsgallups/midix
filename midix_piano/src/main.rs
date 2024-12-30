use bevy::prelude::*;
mod load;
mod piano;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((load::plugin, piano::plugin))
        .run();
}
