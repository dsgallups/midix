#![warn(missing_docs)]
#![cfg_attr(not(feature = "debug"), warn(clippy::print_stdout))]
#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub(crate) mod utils;

mod error;
pub use error::*;

pub mod channel;

pub mod events;
pub mod file;
pub mod file_repr;

pub mod reader;

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

mod target;
pub use target::*;

pub mod prelude {
    #![doc = r#"
        Common re-exports when working with `midix`
    "#]
    pub use crate::{
        channel::*,
        events::*,
        file::*,
        file_repr::{chunk::*, meta::*, track::*, *},
        message::{MidiMessage, channel::*, system::*},
        *,
    };

    pub use crate::reader::{MidiSource, ReadResult, Reader};

    #[allow(unused_imports)]
    pub(crate) use crate::reader::inv_data;

    pub use core::fmt::Display;

    //pub(crate) use crate::utils::*;
}
