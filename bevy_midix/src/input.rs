#![doc = r#"
Contains [`MidiInputPlugin`] and other types to handle input
"#]

use bevy::prelude::Plugin;
use bevy::{prelude::*, tasks::IoTaskPool};
use crossbeam_channel::{Receiver, Sender};
use midir::ConnectErrorKind; // XXX: do we expose this?
pub use midir::{Ignore, MidiInputPort};
use midix::events::{FromLiveEventBytes, LiveEvent};
use std::error::Error;
use std::fmt::Display;
use std::future::Future;
use MidiInputError::{ConnectionError, PortRefreshError};

use crate::asset::{MidiFile, MidiFileLoader};

#[doc = r#"
Inserts [`MidiInputSettings`] and [`MidiInputConnection`] as resource

Input system utilizes the [`PreUpdate`] schedule
"#]
pub struct MidiInputPlugin;

impl Plugin for MidiInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MidiInputSettings>()
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

/// Settings for [`MidiInputPlugin`].
///
/// This resource must be added before [`MidiInputPlugin`] to take effect.
#[derive(Resource, Clone, Debug)]
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

/// [`Resource`] for receiving midi messages.
///
/// Change detection will only fire when its input ports are refreshed.

#[derive(Resource)]
pub struct MidiInput {
    receiver: Receiver<Reply>,
    sender: Sender<Message>,
    ports: Vec<(String, MidiInputPort)>,
}

impl MidiInput {
    /// Update the available input ports.
    ///
    /// This method temporarily disconnects from the current midi port, so
    /// some [`MidiData`] events may be missed.
    ///
    /// Change detection is fired when the ports are refreshed.
    pub fn refresh_ports(&self) {
        info!("Refreshing ports");
        self.sender
            .send(Message::RefreshPorts)
            .expect("Couldn't refresh input ports");
    }

    /// Connects to the given `port`.
    pub fn connect(&self, port: MidiInputPort) {
        self.sender
            .send(Message::ConnectToPort(port))
            .expect("Failed to connect to port");
    }

    /// Disconnects from the current input port.
    pub fn disconnect(&self) {
        self.sender
            .send(Message::DisconnectFromPort)
            .expect("Failed to disconnect from port");
    }

    /// Get the current input ports, and their names.
    #[must_use]
    pub fn ports(&self) -> &Vec<(String, MidiInputPort)> {
        &self.ports
    }
}

/// [`Resource`] for checking whether [`MidiInput`] is
/// connected to any ports.
///
/// Change detection fires whenever the connection changes.
#[derive(Resource, Default)]
pub struct MidiInputConnection {
    connected: bool,
}

impl MidiInputConnection {
    /// Are you connected?
    #[must_use]
    pub fn is_connected(&self) -> bool {
        self.connected
    }
}

/// An [`Event`] for incoming midi data.
#[derive(Resource, Event, Debug)]
pub struct MidiData {
    /// Returns the timestamp of the data
    pub stamp: Option<u64>,

    /// The underlying message of the event
    pub message: LiveEvent<'static>,
}

/// The [`Error`] type for midi input operations, accessible as an [`Event`].
#[derive(Clone, Debug, Event)]
pub enum MidiInputError {
    /// There was something wrong connecting to the input
    ConnectionError(ConnectErrorKind),

    /// Something happened when refreshing the port statuses
    PortRefreshError,
}

impl Error for MidiInputError {}
impl Display for MidiInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            ConnectionError(k) => match k {
                ConnectErrorKind::InvalidPort => {
                    write!(f, "Couldn't (re)connect to input port: invalid port")?;
                }
                ConnectErrorKind::Other(s) => {
                    write!(f, "Couldn't (re)connect to input port: {}", s)?;
                }
            },
            PortRefreshError => write!(f, "Couldn't refresh input ports")?,
        }
        Ok(())
    }
}

fn reply(
    mut input: ResMut<MidiInput>,
    mut conn: ResMut<MidiInputConnection>,
    mut err: EventWriter<MidiInputError>,
    mut midi: EventWriter<MidiData>,
) {
    while let Ok(msg) = input.receiver.try_recv() {
        debug!("Reply received!\n{}", msg.dbg());
        match msg {
            Reply::AvailablePorts(ports) => {
                input.ports = ports;
            }
            Reply::Error(e) => {
                warn!("{}", e);
                err.write(e);
            }
            Reply::Connected => {
                conn.connected = true;
            }
            Reply::Disconnected => {
                conn.connected = false;
            }
            Reply::Midi(m) => {
                midi.write(m);
            }
        }
    }
}

// Core system
fn setup(mut commands: Commands, settings: Res<MidiInputSettings>) {
    let (m_sender, m_receiver) = crossbeam_channel::unbounded::<Message>();
    let (r_sender, r_receiver) = crossbeam_channel::unbounded::<Reply>();

    let thread_pool = IoTaskPool::get();
    thread_pool
        .spawn(MidiInputTask {
            receiver: m_receiver,
            sender: r_sender,
            settings: settings.clone(),
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

enum Message {
    RefreshPorts,
    ConnectToPort(MidiInputPort),
    DisconnectFromPort,
}

enum Reply {
    AvailablePorts(Vec<(String, MidiInputPort)>),
    Error(MidiInputError),
    Connected,
    Disconnected,
    Midi(MidiData),
}

impl Reply {
    pub fn dbg(&self) -> String {
        use Reply::*;
        match self {
            Reply::AvailablePorts(ports) => {
                let mut res = "Available Ports:\n".to_string();
                for (name, port) in ports.iter() {
                    res.push_str(&format!("Name - {}, port id- {}", name, port.id()));
                }
                res
            }
            Error(e) => {
                format!("Error({e:?}")
            }
            Connected => "Connected".to_string(),
            Disconnected => "Disconnected".to_string(),
            Midi(_) => "Midi Data".to_string(),
        }
    }
}

struct MidiInputTask {
    receiver: Receiver<Message>,
    sender: Sender<Reply>,
    settings: MidiInputSettings,
    // Invariant: exactly one of `input` or `connection` is Some
    input: Option<midir::MidiInput>,
    connection: Option<(midir::MidiInputConnection<()>, MidiInputPort)>,
}

impl Future for MidiInputTask {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.input.is_none() && self.connection.is_none() {
            self.input = midir::MidiInput::new(self.settings.client_name).ok();
            self.sender
                .send(get_available_ports(self.input.as_ref().unwrap()))
                .unwrap();
        }

        if let Ok(msg) = self.receiver.recv() {
            use Message::{ConnectToPort, DisconnectFromPort, RefreshPorts};

            match msg {
                ConnectToPort(port) => {
                    let was_connected = self.input.is_none();
                    let s = self.sender.clone();
                    let i = self
                        .input
                        .take()
                        .unwrap_or_else(|| self.connection.take().unwrap().0.close().0);
                    let conn = i.connect(
                        &port,
                        self.settings.port_name,
                        move |stamp, message, _| {
                            let Ok(message) = LiveEvent::from_bytes(message) else {
                                return;
                            };
                            let data = MidiData {
                                stamp: Some(stamp),
                                message,
                            };
                            let _ = s.send(Reply::Midi(data));
                        },
                        (),
                    );
                    match conn {
                        Ok(conn) => {
                            self.sender.send(Reply::Connected).unwrap();
                            self.connection = Some((conn, port));
                            self.input = None;
                        }
                        Err(conn_err) => {
                            self.sender
                                .send(Reply::Error(ConnectionError(conn_err.kind())))
                                .unwrap();
                            if was_connected {
                                self.sender.send(Reply::Disconnected).unwrap();
                            }
                            self.connection = None;
                            self.input = Some(conn_err.into_inner());
                        }
                    }
                }
                DisconnectFromPort => {
                    if let Some((conn, _)) = self.connection.take() {
                        self.input = Some(conn.close().0);
                        self.connection = None;
                        self.sender.send(Reply::Disconnected).unwrap();
                    }
                }
                RefreshPorts => match &self.input {
                    Some(i) => {
                        self.sender.send(get_available_ports(i)).unwrap();
                    }
                    None => {
                        let (conn, port) = self.connection.take().unwrap();
                        let i = conn.close().0;

                        self.sender.send(get_available_ports(&i)).unwrap();

                        let s = self.sender.clone();
                        let conn = i.connect(
                            &port,
                            self.settings.port_name,
                            move |stamp, message, _| {
                                let Ok(message) = LiveEvent::from_bytes(message) else {
                                    return;
                                };
                                let data = MidiData {
                                    stamp: Some(stamp),
                                    message,
                                };
                                let _ = s.send(Reply::Midi(data));
                            },
                            (),
                        );
                        match conn {
                            Ok(conn) => {
                                self.connection = Some((conn, port));
                                self.input = None;
                            }
                            Err(conn_err) => {
                                self.sender
                                    .send(Reply::Error(ConnectionError(conn_err.kind())))
                                    .unwrap();
                                self.sender.send(Reply::Disconnected).unwrap();
                                self.connection = None;
                                self.input = Some(conn_err.into_inner());
                            }
                        }
                    }
                },
            }
        }
        cx.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

// Helper for above.
//
// Returns either Reply::AvailablePorts or Reply::PortRefreshError
// If there's an error getting port names, it's because the available ports changed,
// so it tries again (up to 10 times)
fn get_available_ports(input: &midir::MidiInput) -> Reply {
    for _ in 0..10 {
        let ports = input.ports();
        let ports: Result<Vec<_>, _> = ports
            .into_iter()
            .map(|p| input.port_name(&p).map(|n| (n, p)))
            .collect();
        if let Ok(ports) = ports {
            return Reply::AvailablePorts(ports);
        }
    }
    Reply::Error(PortRefreshError)
}

// A system which debug prints note events
fn debug(mut midi: EventReader<MidiData>) {
    for data in midi.read() {
        debug!("Message: {:?}", data.message);
    }
}
