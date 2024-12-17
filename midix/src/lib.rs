/// All of the errors this crate produces.
#[macro_use]
mod error;

pub mod bytes;
mod channel;
pub mod message;
pub(crate) mod utils;

pub use crate::{
    channel::Channel,
    message::{ChannelVoiceEvent, ChannelVoiceMessage},
};

pub mod prelude {
    pub use crate::bytes::*;
    pub use crate::message::*;
}
