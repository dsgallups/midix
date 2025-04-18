/*

This Sink will send events to another thread that will constantly poll/flush command out to the synth.
*/

mod commands;
pub use commands::*;

mod task;
pub(crate) use task::*;

mod inner;
