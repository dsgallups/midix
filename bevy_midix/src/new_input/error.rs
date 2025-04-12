use bevy::prelude::*;
use midir::ConnectErrorKind; // XXX: do we expose this?
use std::error::Error;
use std::fmt::Display;

/// The [`Error`] type for midi input operations, accessible as an [`Event`].
#[derive(Clone, Debug, Event)]
pub enum MidiInputError {
    /// There was something wrong connecting to the input
    ConnectionError(ConnectErrorKind),

    /// Something happened when refreshing the port statuses
    PortRefreshError,
    /// Invalid state
    InvalidState(String),
}
impl MidiInputError {
    pub fn invalid(msg: impl ToString) -> Self {
        Self::InvalidState(msg.to_string())
    }
}

impl Error for MidiInputError {}
impl Display for MidiInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::ConnectionError(k) => match k {
                ConnectErrorKind::InvalidPort => {
                    write!(f, "Couldn't (re)connect to input port: invalid port")
                }
                ConnectErrorKind::Other(s) => {
                    write!(f, "Couldn't (re)connect to input port: {}", s)
                }
            },
            Self::PortRefreshError => write!(f, "Couldn't refresh input ports"),
            Self::InvalidState(s) => write!(f, "Invalid State: {}", s),
        }
    }
}
