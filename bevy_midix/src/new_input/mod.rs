#![allow(missing_docs)]
use bevy::prelude::*;
use midir::MidiInputPort;

mod settings;
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
pub(crate) enum PortCommand {
    RefreshPorts,
    ConnectToPort(MidiInputPort),
    DisconnectFromPort,
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
    mut commands: EventReader<PortCommand>,
    midi_settings: Res<MidiInputSettings>,
    mut midi_input: ResMut<MidiInput>,
    midir: Res<MidiConnection>,
) {
    if !midir.listening() {
        warn!("Midix cannot evaluate a portcommand if already connected!");
        return;
    }
    for command in commands.read() {
        match command {
            PortCommand::RefreshPorts => {
                midi_input.ports = midir.ports().unwrap();
            }
            PortCommand::ConnectToPort(port) => {
                let connection = midir
                    .0
                    .connect(
                        port,
                        midi_settings.port_name,
                        move |timestamp, message, _| {
                            //todo
                            //todo
                        },
                        (),
                    )
                    .unwrap();
            }
            PortCommand::DisconnectFromPort => {
                todo!()
            }
        }
    }
}
