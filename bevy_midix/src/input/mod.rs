#![doc = r#"
Contains [`MidiInputPlugin`] and other types to handle input
"#]

pub use midir::{Ignore, MidiInputPort};

mod connection;
pub use connection::*;

mod plugin;
pub use plugin::*;

mod error;
pub use error::*;

mod input_resource;
pub use input_resource::*;

mod reply;
use reply::*;

mod data;
pub use data::*;

mod task;
pub use task::*;
