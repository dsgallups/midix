use bevy::prelude::*;
/// [`Resource`] for checking whether [`MidiInput`] is
/// connected to any ports.
///
/// Change detection fires whenever the connection changes.
#[derive(Resource, Default)]
pub struct MidiInputConnection {
    pub connected: bool,
}

impl MidiInputConnection {
    /// Are you connected?
    #[must_use]
    pub fn is_connected(&self) -> bool {
        self.connected
    }
}
