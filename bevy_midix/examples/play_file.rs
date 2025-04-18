use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};

use bevy_midix::{asset::MidiFile, prelude::*};

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
struct CrabRave(Handle<MidiFile>);

/// Take a look here for some soundfonts:
///
/// <https://sites.google.com/site/soundfonts4u/>
fn load_sf2(mut commands: Commands, asset_server: Res<AssetServer>, mut synth: ResMut<Synth>) {
    synth.use_soundfont(asset_server.load("8bitsf.sf2"));
    let handle = asset_server.load::<MidiFile>("21Guns.mid");
    commands.insert_resource(CrabRave(handle));
}

fn play_song(
    synth: Res<Synth>,
    cr: Res<CrabRave>,
    assets: Res<Assets<MidiFile>>,
    mut run: Local<bool>,
) {
    if *run || !synth.is_ready() {
        return;
    }

    let Some(crab_rave) = assets.get(&cr.0) else {
        return;
    };
    println!("pushing audio!");

    synth.push_audio(crab_rave);

    *run = true;
}
