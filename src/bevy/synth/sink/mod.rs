/*

This Sink will send events to another thread that will constantly poll/flush command out to the synth.
*/

mod commands;
pub(crate) use commands::*;

mod task;
#[allow(unused_imports)]
pub(crate) use task::*;
