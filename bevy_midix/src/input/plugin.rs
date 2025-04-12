use bevy::{prelude::*, tasks::IoTaskPool};
pub use midir::Ignore;

use crate::asset::{MidiFile, MidiFileLoader};

use super::{
    Message, MidiData, MidiInput, MidiInputConnection, MidiInputError, MidiInputTask, MidirReply,
};

/// Settings for [`MidiInputPlugin`].
#[derive(Resource, Clone, Copy, Debug)]
pub struct MidiInputSettings {
    /// The name of the listening client
    pub client_name: &'static str,

    /// The port name of the listening client
    pub port_name: &'static str,

    /// Describe what events you want to ignore.
    ///
    /// If you don't care about System Exclusive messages
    /// (manufacturer specific messages to their proprietary devices),
    /// set this value to [`Ignore::Sysex`].
    pub ignore: Ignore,
}

impl Default for MidiInputSettings {
    /// Assigns client name and port name to `bevy_midix`
    ///
    /// ignore is set to [`Ignore::None`]
    fn default() -> Self {
        Self {
            client_name: "bevy_midix",
            port_name: "bevy_midix",
            ignore: Ignore::None,
        }
    }
}

#[doc = r#"
Inserts [`MidiInputSettings`] and [`MidiInputConnection`] as resource

Input system utilizes the [`PreUpdate`] schedule
"#]
#[derive(Clone, Copy, Debug, Default)]
pub struct MidiInputPlugin {
    settings: MidiInputSettings,
}

impl Plugin for MidiInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.settings)
            .init_resource::<MidiInputConnection>()
            .init_asset::<MidiFile>()
            .init_asset_loader::<MidiFileLoader>()
            .add_event::<MidiInputError>()
            .add_event::<MidiData>()
            .add_systems(Startup, setup)
            .add_systems(PreUpdate, reply)
            .add_systems(Update, debug);
    }
}

// Core system
fn setup(mut commands: Commands, settings: Res<MidiInputSettings>) {
    let (m_sender, m_receiver) = crossbeam_channel::unbounded::<Message>();
    let (r_sender, r_receiver) = crossbeam_channel::unbounded::<MidirReply>();

    let thread_pool = IoTaskPool::get();
    thread_pool
        .spawn(MidiInputTask {
            receiver: m_receiver,
            sender: r_sender,
            settings: *settings,
            input: None,
            connection: None,
        })
        .detach();

    commands.insert_resource(MidiInput {
        sender: m_sender,
        receiver: r_receiver,
        ports: Vec::new(),
    });
}

fn reply(
    mut input: ResMut<MidiInput>,
    mut conn: ResMut<MidiInputConnection>,
    mut err: EventWriter<MidiInputError>,
    mut midi: EventWriter<MidiData>,
) {
    while let Ok(msg) = input.receiver.try_recv() {
        debug!("MidirReply received!\n{}", msg.dbg());
        match msg {
            MidirReply::AvailablePorts(ports) => {
                input.ports = ports;
            }
            MidirReply::Error(e) => {
                warn!("{}", e);
                err.write(e);
            }
            MidirReply::Connected => {
                conn.connected = true;
            }
            MidirReply::Disconnected => {
                conn.connected = false;
            }
            MidirReply::Midi(m) => {
                midi.write(m);
            }
        }
    }
}

// A system which debug prints note events
fn debug(mut midi: EventReader<MidiData>) {
    for data in midi.read() {
        debug!("Message: {:?}", data.message);
    }
}
