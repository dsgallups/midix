use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};
use midir::MidiInputPort;

use super::{MidirReply, UserMessage};

/// [`Resource`] for receiving midi messages.
///
/// Change detection will only fire when its input ports are refreshed.
#[derive(Resource)]
pub struct OldMidiInput {
    // used to talk to the midi input task
    pub(crate) receiver: Receiver<MidirReply>,
    // used to talk to the midi input task
    pub(crate) sender: Sender<UserMessage>,
    pub(crate) ports: Vec<(String, MidiInputPort)>,
}

impl OldMidiInput {
    /// Update the available input ports.
    ///
    /// This method temporarily disconnects from the current midi port, so
    /// some [`MidiData`] events may be missed.
    ///
    /// Change detection is fired when the ports are refreshed.
    pub fn refresh_ports(&self) {
        info!("Refreshing ports");
        self.sender
            .send(UserMessage::RefreshPorts)
            .expect("Couldn't refresh input ports");
    }

    /// Connects to the given `port`.
    pub fn connect(&self, port: MidiInputPort) {
        self.sender
            .send(UserMessage::ConnectToPort(port))
            .expect("Failed to connect to port");
    }

    /// Disconnects from the current input port.
    pub fn disconnect(&self) {
        self.sender
            .send(UserMessage::DisconnectFromPort)
            .expect("Failed to disconnect from port");
    }

    /// Get the current input ports, and their names.
    #[must_use]
    pub fn ports(&self) -> &Vec<(String, MidiInputPort)> {
        &self.ports
    }
}
