#![doc = r#"
a plugin and types for handling MIDI input
"#]
use crossbeam_channel::TryRecvError;

mod connection;
pub use connection::*;

mod error;
pub use error::*;

mod plugin;
pub use plugin::*;

use bevy::prelude::*;
use midir::MidiInputPort;

use crate::MidiSettings;

// you can't actually have multiple MidiInputs on one device, it's really strange.
enum MidiInputState {
    Listening(midir::MidiInput),
    Active(MidiInputConnection),
}
/// SAFETY: This applies to linux alsa.
///
/// There is only one instance of MidiInput at any time using this crate.
///
/// However, this may not satisfy the requirements for safety. If another instance of
/// MidiInput exists in the external program, then UB is possible.
///
/// Therefore, the assumption is, that when using this crate, that the user
/// will NOT instantiate another [`midir::MidiInput`] at any point while
/// [`MidiInput`] has been inserted as a resource
unsafe impl Sync for MidiInputState {}

/// The central resource for interacting with midi inputs
///
/// `MidiInput` does many things:
/// - Fetches a list of ports with connected midi devices
/// - Allows one to connect to a particular midi device and read output
/// - Close that connection and search for other devices
#[derive(Resource)]
pub struct MidiInput {
    settings: MidiSettings,
    state: Option<MidiInputState>,
    ports: Vec<MidiInputPort>,
}

impl MidiInput {
    /// Creates a new midi input with the provided settings. This is done automatically
    /// by [`MidiInputPlugin`].
    pub fn new(settings: MidiSettings) -> Self {
        let listener = match midir::MidiInput::new(settings.client_name) {
            Ok(input) => input,
            Err(e) => {
                panic!("Error initializing midi input! {e:?}");
            }
        };
        let ports = listener.ports();
        Self {
            state: Some(MidiInputState::Listening(listener)),
            settings,
            ports,
        }
    }

    /// Return a list of ports updated since calling [`MidiInput::new`] or
    /// [`MidiInput::refresh_ports`]
    pub fn ports(&self) -> &[MidiInputPort] {
        &self.ports
    }
    /// Attempts to connects to the port at the given index returned by [`MidiInput::ports`]
    ///
    /// # Errors
    /// - If already connected to a device
    /// - If the index is out of bounds
    /// - An input connection cannot be established
    pub fn connect_to_index(&mut self, index: usize) -> Result<(), MidiInputError> {
        if self
            .state
            .as_ref()
            .is_none_or(|s| matches!(s, MidiInputState::Active(_)))
        {
            return Err(MidiInputError::invalid(
                "Cannot connect: not currently active!",
            ));
        }
        let Some(port) = self.ports.get(index) else {
            return Err(MidiInputError::port_not_found(
                "Port was not found at {index}!",
            ));
        };

        let MidiInputState::Listening(listener) = self.state.take().unwrap() else {
            unreachable!()
        };

        self.state = Some(MidiInputState::Active(
            MidiInputConnection::new(listener, port, self.settings.port_name).unwrap(),
        ));
        Ok(())
    }

    /// A method you should call if [`MidiInput::is_listening`] and [`MidiInput::is_active`] are both false.
    pub fn reset(&mut self) {
        let listener = match midir::MidiInput::new(self.settings.client_name) {
            Ok(input) => input,
            Err(e) => {
                error!("Failed to reset listening state! {e:?}");
                return;
            }
        };
        self.state = Some(MidiInputState::Listening(listener));
    }
    /// Attempts to connects to the passed port
    ///
    /// # Errors
    /// - If already connected to a device
    /// - An input connection cannot be established
    pub fn connect_to_port(&mut self, port: &MidiInputPort) -> Result<(), MidiInputError> {
        if self
            .state
            .as_ref()
            .is_none_or(|s| matches!(s, MidiInputState::Active(_)))
        {
            return Err(MidiInputError::invalid(
                "Cannot connect: not currently active!",
            ));
        }
        let MidiInputState::Listening(listener) = self.state.take().unwrap() else {
            unreachable!()
        };

        self.state = Some(MidiInputState::Active(
            MidiInputConnection::new(listener, port, self.settings.port_name).unwrap(),
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
    /// - An input connection cannot be established
    pub fn connect_to_id(&mut self, id: String) -> Result<(), MidiInputError> {
        if self
            .state
            .as_ref()
            .is_none_or(|s| matches!(s, MidiInputState::Active(_)))
        {
            return Err(MidiInputError::invalid(
                "Cannot connect: not currently active!",
            ));
        }
        let MidiInputState::Listening(listener) = self.state.take().unwrap() else {
            unreachable!()
        };
        let Some(port) = listener.find_port_by_id(id.clone()) else {
            return Err(MidiInputError::port_not_found(id));
        };
        self.state = Some(MidiInputState::Active(
            MidiInputConnection::new(listener, &port, self.settings.port_name).unwrap(),
        ));
        Ok(())
    }
    /// True if a device is currently connected
    pub fn is_active(&self) -> bool {
        self.state
            .as_ref()
            .is_some_and(|s| matches!(s, MidiInputState::Active(_)))
    }

    /// True if input is waiting to connect to a device
    pub fn is_listening(&self) -> bool {
        self.state
            .as_ref()
            .is_some_and(|s| matches!(s, MidiInputState::Listening(_)))
    }

    /// Refreshes the available port list
    ///
    /// Does nothing if [`MidiInput::is_active`] is true
    pub fn refresh_ports(&mut self) {
        let Some(MidiInputState::Listening(listener)) = &self.state else {
            return;
        };
        self.ports = listener.ports();
    }

    /// Disconnects from the active device
    ///
    /// Does nothing if the [`MidiInput::is_listening`] is true.
    pub fn disconnect(&mut self) {
        if self
            .state
            .as_ref()
            .is_none_or(|s| matches!(s, MidiInputState::Listening(_)))
        {
            return;
        }
        let MidiInputState::Active(conn) = self.state.take().unwrap() else {
            unreachable!()
        };
        let listener = conn.close();
        self.state = Some(MidiInputState::Listening(listener));
    }

    /// will return data if connected. Note, this CONSUMES the event.
    ///
    /// You will need to propagate this data out to other systems if need be.
    pub fn read(&self) -> Result<MidiData, TryRecvError> {
        let Some(MidiInputState::Active(conn)) = &self.state else {
            return Err(TryRecvError::Disconnected);
        };
        conn.read()
    }
}
