#![warn(missing_docs)]
#![warn(clippy::print_stdout)]
#![doc = include_str!("../README.md")]

use std::io::{self, ErrorKind};

#[macro_use]
mod error;

pub mod reader;
pub(crate) mod utils;

pub mod channel;

pub mod events;
pub mod file;
mod pitch_bend;
pub use pitch_bend::*;

mod program;
pub use program::*;

mod velocity;
pub use velocity::*;

mod key;
pub use key::*;

mod controller;
pub use controller::*;

mod byte;
pub use byte::*;

pub mod message;

mod song_position_pointer;
pub use song_position_pointer::*;

pub(crate) trait ReadDataBytesExt {
    fn get_byte(&self, byte: usize) -> Result<&u8, io::Error>;
}

impl ReadDataBytesExt for &[u8] {
    fn get_byte(&self, byte: usize) -> Result<&u8, io::Error> {
        self.get(byte).ok_or(io_error!(
            ErrorKind::InvalidInput,
            "Data not accessible for message!"
        ))
    }
}

pub mod prelude {
    #![doc = r#"
        Common re-exports when working with `midix`
    "#]
    pub use crate::{
        channel::*,
        events::*,
        file::{chunk::*, meta::*, track::*, *},
        message::{channel::*, system::*, MidiMessage},
        *,
    };

    pub use crate::reader::{ReadResult, Reader};

    #[allow(unused_imports)]
    pub(crate) use crate::reader::{inv_data, inv_input, unexp_eof};

    pub use core::fmt::Display;

    pub(crate) use crate::utils::*;
}
