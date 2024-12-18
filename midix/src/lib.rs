/// All of the errors this crate produces.
#[macro_use]
mod error;

pub mod bytes;
pub mod channel;
pub mod message;
pub(crate) mod utils;

pub mod prelude {
    pub use crate::bytes::*;
    pub use crate::channel::Channel;
    pub use crate::message::{controller::*, key::*, pitch_bend::*, program::*, velocity::*, *};
    pub use core::fmt::Display;

    pub(crate) use crate::utils::*;
}
