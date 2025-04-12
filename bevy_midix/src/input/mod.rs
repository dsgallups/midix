#![allow(missing_docs)]

mod settings;
use crossbeam_channel::TryRecvError;
pub use settings::*;

mod connection;
pub use connection::*;

mod error;
pub use error::*;

mod plugin;
pub use plugin::*;

use bevy::prelude::*;
use midir::MidiInputPort;

// you can't actually have multiple MidiInputs on one device, it's really strange.
pub enum MidiInputState {
    Listening(midir::MidiInput),
    Active(MidiInputConnection),
}
#[derive(Resource)]
pub struct MidiInput {
    settings: MidiInputSettings,
    state: Option<MidiInputState>,
    ports: Vec<MidiInputPort>,
}

impl MidiInput {
    pub fn new(settings: MidiInputSettings) -> Self {
        let listener = match midir::MidiInput::new(settings.client_name) {
            Ok(input) => input,
            Err(e) => {
                panic!("Error initializing midi input for port refresh: {e:?}");
            }
        };
        let ports = listener.ports();
        Self {
            state: Some(MidiInputState::Listening(listener)),
            settings,
            ports,
        }
    }
    pub fn ports(&self) -> &[MidiInputPort] {
        &self.ports
    }
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

    //pub fn connect_to_port(&mut self, port: )
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
    pub fn is_active(&self) -> bool {
        self.state
            .as_ref()
            .is_some_and(|s| matches!(s, MidiInputState::Active(_)))
    }
    pub fn is_listening(&self) -> bool {
        self.state
            .as_ref()
            .is_some_and(|s| matches!(s, MidiInputState::Listening(_)))
    }

    pub fn refresh_ports(&mut self) {
        let Some(MidiInputState::Listening(listener)) = &self.state else {
            return;
        };
        self.ports = listener.ports();
    }
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

    /// will return data if connected
    pub fn read(&self) -> Result<MidiData, TryRecvError> {
        let Some(MidiInputState::Active(conn)) = &self.state else {
            return Err(TryRecvError::Disconnected);
        };
        conn.read()
    }
}
