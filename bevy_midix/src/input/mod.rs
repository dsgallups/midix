#![doc = r#"
Contains [`MidiInputPlugin`] and other types to handle input
"#]

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

mod connection;
pub use connection::*;

mod plugin;
pub use plugin::*;

mod error;
pub use error::*;

mod input_resource;
pub use input_resource::*;

mod reply;
pub use reply::*;

mod data;
pub use data::*;

mod task;
pub use task::*;
