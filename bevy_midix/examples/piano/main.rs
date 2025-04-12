use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_midix::prelude::*;

mod ui;

///Creates a 2d Piano Keyboard and plays the sound on press.
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: Level::INFO,
                filter: "bevy_midix=DEBUG".to_string(),
                ..default()
            }),
            MidiPlugin::default(),
            ui::plugin,
        ))
        .add_systems(Startup, load_sf2)
        .run();
}
/// Note: you need to bring your own soundfont file.
///
/// sf2s are generally huge, so I added those to the gitignore.
///
/// Take a look here for some soundfonts:
///
/// <https://sites.google.com/site/soundfonts4u/>
fn load_sf2(asset_server: Res<AssetServer>, mut synth: ResMut<Synth>) {
    synth.use_soundfont(asset_server.load("soundfont.sf2"));
}
