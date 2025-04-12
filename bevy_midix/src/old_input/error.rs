use bevy::prelude::*;
use midir::ConnectErrorKind; // XXX: do we expose this?
use std::error::Error;
use std::fmt::Display;
use MidiInputError::{ConnectionError, PortRefreshError};

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
