#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub mod asset;
pub mod input;
pub mod output;

pub use midix;

/// Commonly re-exported types
pub mod prelude {
    #[allow(ambiguous_glob_reexports)]
    pub use crate::{asset::*, input::*, output::*, *};
}
