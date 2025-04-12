use bevy::prelude::*;
use midir::ConnectError; // XXX: do we expose this?
use thiserror::Error;

/// The [`Error`] type for midi input operations, accessible as an [`Event`].
#[derive(Debug, Event, Error)]
pub enum MidiInputError {
    /// There was something wrong connecting to the input
    #[error("Couldn't reconnect to input port: {0}")]
    ConnectionError(#[from] ConnectError<midir::MidiInput>),

    /// The port, passed by id, was not found.
    #[error("Port not found (id: {0}")]
    PortNotFound(String),

    /// Something happened when refreshing the port statuses
    #[error("Couldn't refersh input ports")]
    PortRefreshError,
    /// Invalid state
    #[error("Invalid State: {0}")]
    InvalidState(String),
}
impl MidiInputError {
    pub(crate) fn invalid(msg: impl ToString) -> Self {
        Self::InvalidState(msg.to_string())
    }
    pub(crate) fn port_not_found(id: impl Into<String>) -> Self {
        Self::PortNotFound(id.into())
    }
}
