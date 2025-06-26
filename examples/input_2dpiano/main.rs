use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use midix::prelude::*;

mod key_input;

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
            MidiPlugin::default(),
        ))
        .add_event::<ExampleInputEvent>()
        .add_plugins((key_input::plugin, ui::plugin))
        .add_systems(Startup, add_soundfont)
        .add_systems(PreUpdate, handle_mididata)
        .run();
}
/// used to propagate the event to the piano ui
#[derive(Event)]
pub struct ExampleInputEvent {
    pub voice: VoiceEvent,
}

/// Take a look here for some soundfonts:
///
/// <https://sites.google.com/site/soundfonts4u/>
fn add_soundfont(asset_server: Res<AssetServer>, mut synth: ResMut<Synth>) {
    // include the soundfont file
    synth.use_soundfont(asset_server.load("soundfont.sf2"));
}

fn handle_mididata(
    midi_input: Res<MidiInput>,
    synth: Res<Synth>,
    mut ev: EventWriter<ExampleInputEvent>,
) {
    while let Ok(data) = midi_input.read() {
        let LiveEvent::ChannelVoice(event) = data.message else {
            continue;
        };

        info!("Data: {:?}", data.message);
        //todo
        _ = synth.push_event(event);
        ev.write(ExampleInputEvent {
            voice: *event.event(),
        });
    }
}
