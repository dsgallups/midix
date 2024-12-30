#![doc = include_str!("../README.md")]

pub mod input;
pub mod output;

pub mod prelude {
    #[allow(ambiguous_glob_reexports)]
    pub use crate::{input::*, output::*, *};
    pub use midix::prelude::*;
}
