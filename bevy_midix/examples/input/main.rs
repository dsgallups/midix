use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_midix::prelude::*;

mod midi_stuff;
mod ui;

/// Waits midi input and plays a sound on press.
///
/// Note: due to the size of soundfont files and the lack of optimization
/// for running this example, you should run this with example with `--release`
///
/// i.e.
/// ```console
/// cargo run --example 2dpiano --release
/// ```
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: Level::INFO,
                ..default()
            }),
            MidiPlugin {
                input: false,
                ..Default::default()
            },
        ))
        .add_plugins((midi_stuff::plugin, ui::plugin))
        .add_systems(Startup, add_soundfont)
        .run();
}

/// Note: you need to bring your own soundfont file.
///
/// sf2s are generally huge, so I added those to the gitignore.
///
/// Take a look here for some soundfonts:
///
/// <https://sites.google.com/site/soundfonts4u/>
fn add_soundfont(asset_server: Res<AssetServer>, mut synth: ResMut<Synth>) {
    // include the soundfont file
    synth.use_soundfont(asset_server.load("soundfont.sf2"));
}
