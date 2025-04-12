use bevy::prelude::*;
use midix::events::LiveEvent;

// An [`Event`] for incoming midi data.
#[derive(Resource, Event, Debug)]
pub struct MidiData {
    /// Returns the timestamp of the data
    pub stamp: Option<u64>,

    /// The underlying message of the event
    pub message: LiveEvent<'static>,
}
