#![allow(missing_docs)]
use bevy::prelude::*;
use midir::MidiInputPort;

mod settings;
use midix::events::LiveEvent;
pub use settings::*;

mod midir_stuff;
pub use midir_stuff::*;

mod connection;
pub use connection::*;

mod error;
pub use error::*;

#[doc = r#"
Inserts [`MidiInputSettings`] and [`MidiInputConnection`] as resource

Input system utilizes the [`PreUpdate`] schedule
"#]
#[derive(Clone, Copy, Debug, Default)]
pub struct MidiInputPlugin {
    settings: MidiInputSettings,
}

#[derive(Event)]
pub(crate) enum MidiInputEvent {
    RefreshPorts,
    ConnectToPort(String),
    Disconnect(Entity),
}

impl Plugin for MidiInputPlugin {
    fn build(&self, app: &mut App) {
        let midir_input = MidiConnection::new(self.settings.client_name);
        app.init_resource::<MidiInput>()
            .insert_resource(self.settings)
            .insert_resource(midir_input)
            //.add_systems(PreUpdate, update_available_ports)
            .add_systems(PostUpdate, handle_port_commands);
    }
}

#[derive(Resource, Default)]
pub struct MidiInput {
    ports: Vec<MidiInputPort>,
}

fn handle_port_commands(
    mut commands: Commands,
    mut events: EventReader<MidiInputEvent>,
    midi_settings: Res<MidiInputSettings>,
    mut midi_input: ResMut<MidiInput>,
) {
    for command in events.read() {
        match command {
            MidiInputEvent::RefreshPorts => {
                let midir_input = match midir::MidiInput::new(midi_settings.client_name) {
                    Ok(input) => input,
                    Err(e) => {
                        error!("Error initializing midi input for port refresh: {e:?}");
                        continue;
                    }
                };
                midi_input.ports = midir_input.ports();
            }
            MidiInputEvent::ConnectToPort(port_id) => {
                let midir_input = match midir::MidiInput::new(midi_settings.client_name) {
                    Ok(input) => input,
                    Err(e) => {
                        error!("Error initializing midi input for port refresh: {e:?}");
                        continue;
                    }
                };
                let connection = match MidiInputConnection::new(
                    midir_input,
                    port_id.clone(),
                    midi_settings.port_name,
                ) {
                    Ok(conn) => conn,
                    Err(e) => {
                        error!("{e:?}");
                        continue;
                    }
                };
                commands.spawn(connection);
            }
            MidiInputEvent::Disconnect(conn) => {
                commands.entity(*conn).despawn();
            }
        }
    }
}

// TODO:
// In the future, we'll want the input to trigger an event for instant use rather
// than updating per frame in our schedules.
// #[derive(Event)]
// struct SomeTrigger;

// fn sandbox(mut commands: Commands) {
//     let midir_input = match midir::MidiInput::new("foo") {
//         Ok(input) => input,
//         Err(e) => {
//             error!("Error initializing midi input for port refresh: {e:?}");
//             return;
//         }
//     };

//     let Some(port) = midir_input.find_port_by_id("bar".to_string()) else {
//         return;
//     };
//     let conn = midir_input
//         .connect(
//             &port,
//             "foobar",
//             {
//                 move |_timestamp, _data, _| {
//                     commands.trigger(SomeTrigger);
//                     //todo
//                     todo!()
//                 }
//             },
//             (),
//         )
//         .unwrap();
// }
