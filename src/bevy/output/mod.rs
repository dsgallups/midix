#![doc = r#"
A plugin and types for handling MIDI output
"#]

mod connection;
use connection::*;

mod error;
pub use error::*;

mod plugin;
pub use plugin::*;

use bevy::prelude::*;
use midir::{MidiOutputPort, SendError};

use crate::{MidiMessageBytes, bevy::settings::MidiSettings};

enum MidiOutputState {
    Listening(midir::MidiOutput),
    Active(MidiOutputConnection),
}
/// SAFETY: This applies to linux alsa.
///
/// There is only one instance of MidiOutput at any time using this crate.
///
/// However, this may not satisfy the requirements for safety. If another instance of
/// MidiOutput exists in the external program, then UB is possible.
///
/// Therefore, the assumption is, that when using this crate, that the user
/// will NOT instantiate another [`midir::MidiOutput`] at any point while
/// [`MidiOutput`] has been inserted as a resource
unsafe impl Sync for MidiOutputState {}

/// The central resource for interacting with midi output devices
///
/// `MidiOutput` does many things:
/// - Fetches a list of ports with connected midi devices
/// - Allows one to connect to a particular midi device and send output
/// - Close that connection and search for other devices
#[derive(Resource)]
pub struct MidiOutput {
    settings: MidiSettings,
    state: Option<MidiOutputState>,
    ports: Vec<MidiOutputPort>,
}

/// SAFETY:
///
/// `JsValue`s in WASM cannot be `Send`: <https://github.com/rustwasm/wasm-bindgen/pull/955>
///
/// Quote:
/// > The JsValue type wraps a slab/heap of js objects which is managed by
/// > the wasm-bindgen shim, and everything here is not actually able to cross
/// > any thread boundaries.
///
/// Therefore, `MidiOutput` nor `MidiInput` should not be able to implement Send and Sync.
///
/// HOWEVER: Because the main scheduler does not run on worker threads, it is safe,
/// for the wasm target, to implement Send (until this issue is resolved.)
/// <https://github.com/bevyengine/bevy/issues/4078>
#[cfg(all(target_arch = "wasm32", target_os = "unknown", feature = "web"))]
unsafe impl Send for MidiOutput {}
/// SAFETY:
///
/// See [`MidiOutput`]'s Send implementation
#[cfg(all(target_arch = "wasm32", target_os = "unknown", feature = "web"))]
unsafe impl Sync for MidiOutput {}

impl MidiOutput {
    /// Creates a new midi output with the provided settings. This is done automatically
    /// by [`MidiOutputPlugin`].
    pub fn new(settings: MidiSettings) -> Self {
        let listener = match midir::MidiOutput::new(settings.client_name) {
            Ok(output) => output,
            Err(e) => {
                panic!("Error initializing midi output! {e:?}");
            }
        };
        let ports = listener.ports();
        Self {
            state: Some(MidiOutputState::Listening(listener)),
            settings,
            ports,
        }
    }

    /// Return a list of ports updated since calling [`MidiOutput::new`] or
    /// [`MidiOutput::refresh_ports`]
    pub fn ports(&self) -> &[MidiOutputPort] {
        &self.ports
    }
    /// Attempts to connects to the port at the given index returned by [`MidiOutput::ports`]
    ///
    /// # Errors
    /// - If already connected to a device
    /// - If the index is out of bounds
    /// - An output connection cannot be established
    pub fn connect_to_index(&mut self, index: usize) -> Result<(), MidiOutputError> {
        if self
            .state
            .as_ref()
            .is_none_or(|s| matches!(s, MidiOutputState::Active(_)))
        {
            return Err(MidiOutputError::invalid(
                "Cannot connect: not currently active!",
            ));
        }
        let Some(port) = self.ports.get(index) else {
            return Err(MidiOutputError::port_not_found(
                "Port was not found at {index}!",
            ));
        };

        let MidiOutputState::Listening(listener) = self.state.take().unwrap() else {
            unreachable!()
        };

        self.state = Some(MidiOutputState::Active(
            MidiOutputConnection::new(listener, port, self.settings.port_name).unwrap(),
        ));
        Ok(())
    }

    /// A method you should call if [`MidiOutput::is_listening`] and [`MidiOutput::is_active`] are both false.
    pub fn reset(&mut self) {
        let listener = match midir::MidiOutput::new(self.settings.client_name) {
            Ok(output) => output,
            Err(e) => {
                error!("Failed to reset listening state! {e:?}");
                return;
            }
        };
        self.state = Some(MidiOutputState::Listening(listener));
    }
    /// Attempts to connects to the passed port
    ///
    /// # Errors
    /// - If already connected to a device
    /// - An output connection cannot be established
    pub fn connect_to_port(&mut self, port: &MidiOutputPort) -> Result<(), MidiOutputError> {
        if self
            .state
            .as_ref()
            .is_none_or(|s| matches!(s, MidiOutputState::Active(_)))
        {
            return Err(MidiOutputError::invalid(
                "Cannot connect: not currently active!",
            ));
        }
        let MidiOutputState::Listening(listener) = self.state.take().unwrap() else {
            unreachable!()
        };

        self.state = Some(MidiOutputState::Active(
            MidiOutputConnection::new(listener, port, self.settings.port_name).unwrap(),
        ));
        Ok(())
    }
    /// Attempts to connects to the passed port
    ///
    /// # Errors
    /// - If already connected to a device
    /// - If the port ID cannot be currently found
    ///   - Note that this case can occur if you have not refreshed ports
    ///     and the device is no longer available.
    /// - An output connection cannot be established
    pub fn connect_to_id(&mut self, id: String) -> Result<(), MidiOutputError> {
        if self
            .state
            .as_ref()
            .is_none_or(|s| matches!(s, MidiOutputState::Active(_)))
        {
            return Err(MidiOutputError::invalid(
                "Cannot connect: not currently active!",
            ));
        }
        let MidiOutputState::Listening(listener) = self.state.take().unwrap() else {
            unreachable!()
        };
        let Some(port) = listener.find_port_by_id(id.clone()) else {
            return Err(MidiOutputError::port_not_found(id));
        };
        self.state = Some(MidiOutputState::Active(
            MidiOutputConnection::new(listener, &port, self.settings.port_name).unwrap(),
        ));
        Ok(())
    }
    /// True if a device is currently connected
    pub fn is_active(&self) -> bool {
        self.state
            .as_ref()
            .is_some_and(|s| matches!(s, MidiOutputState::Active(_)))
    }

    /// True if output is waiting to connect to a device
    pub fn is_listening(&self) -> bool {
        self.state
            .as_ref()
            .is_some_and(|s| matches!(s, MidiOutputState::Listening(_)))
    }

    /// Refreshes the available port list
    ///
    /// Does nothing if [`MidiOutput::is_active`] is true
    pub fn refresh_ports(&mut self) {
        let Some(MidiOutputState::Listening(listener)) = &self.state else {
            return;
        };
        self.ports = listener.ports();
    }

    /// Disconnects from the active device
    ///
    /// Does nothing if the [`MidiOutput::is_listening`] is true.
    pub fn disconnect(&mut self) {
        if self
            .state
            .as_ref()
            .is_none_or(|s| matches!(s, MidiOutputState::Listening(_)))
        {
            return;
        }
        let MidiOutputState::Active(conn) = self.state.take().unwrap() else {
            unreachable!()
        };
        let listener = conn.close();
        self.state = Some(MidiOutputState::Listening(listener));
    }

    /// Sends valid midi bytes to the midi output.
    ///
    /// Errors if [`MidiOutput::is_listening`] is true.
    pub fn send(&mut self, message: impl Into<MidiMessageBytes>) -> Result<(), SendError> {
        let Some(MidiOutputState::Active(conn)) = &mut self.state else {
            return Err(SendError::Other("Disconnected."));
        };
        conn.send(message)
    }
}
