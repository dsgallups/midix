use bevy::{
    asset::AssetMetaCheck,
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_midix::{midix::prelude::*, prelude::*};

mod key_input;

mod ui;

//Note: firefox by default doesn't ask
//the user for MIDI input permissions.
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy game".to_string(), // ToDo
                        // Bind to canvas included in `index.html`
                        canvas: Some("#bevy".to_owned()),
                        fit_canvas_to_parent: true,
                        // Tells wasm not to override default event handling, like F5 and Ctrl+R
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: Level::INFO,
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
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

/// Take a look here for some soundfonts:
///
/// <https://sites.google.com/site/soundfonts4u/>
fn add_soundfont(asset_server: Res<AssetServer>, mut synth: ResMut<Synth>) {
    // include the soundfont file
    synth.use_soundfont(asset_server.load("soundfont.sf2"));
}

/// used to propagate the event to the piano ui
#[derive(Event)]
pub struct ExampleInputEvent {
    pub voice: VoiceEvent,
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
        synth.handle_event(event);
        ev.write(ExampleInputEvent {
            voice: *event.event(),
        });
    }
}
