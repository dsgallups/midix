use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_midix::prelude::*;

mod ui;

#[doc = r#"
Creates a 2d Piano Keyboard and plays the sound on press.

TODO

"#]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: Level::WARN,
                filter: "bevy_midix=debug".to_string(),
                ..default()
            }),
            //MidiPlugin::default(),
            ui::plugin,
        ))
        .run();
}
