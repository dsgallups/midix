#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub mod asset;
pub mod input;
pub mod output;

mod midix {
    pub use midix::prelude::*;
}

/// Commonly re-exported types
pub mod prelude {
    #[allow(ambiguous_glob_reexports)]
    pub use crate::{asset::*, input::*, midix::*, output::*};
}
