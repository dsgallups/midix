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
        .add_systems(Update, play_song)
        .run();
}

#[derive(Resource)]
struct LoadedFile(Handle<MidiFile>);

/// Take a look here for some soundfonts:
///
/// <https://sites.google.com/site/soundfonts4u/>
fn load_sf2(mut commands: Commands, asset_server: Res<AssetServer>, mut synth: ResMut<Synth>) {
    synth.use_soundfont(asset_server.load("8bitsf.SF2"));
    let handle = asset_server.load::<MidiFile>("Africa.mid");
    commands.insert_resource(LoadedFile(handle));
}

fn play_song(
    mut synth: ResMut<Synth>,
    file: Res<LoadedFile>,
    assets: Res<Assets<MidiFile>>,
    mut run: Local<bool>,
) {
    if *run || !synth.is_ready() {
        return;
    }

    let Some(file) = assets.get(&file.0) else {
        return;
    };
    println!("pushing audio!");

    synth.push_audio(file.to_song()).unwrap();

    *run = true;
}
