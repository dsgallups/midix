use bevy::prelude::*;
pub use midir::Ignore;

/// Settings for [`MidiInputPlugin`](crate::prelude::MidiInputPlugin) and [`MidiOutputPlugin`](crate::prelude::MidiOutputPlugin).
#[derive(Resource, Clone, Copy, Debug)]
pub struct MidiSettings {
    /// The name of the listening client
    pub client_name: &'static str,

    /// The port name of the listening client.
    ///
    /// This is appended to the port name of a connection essentially.
    pub port_name: &'static str,

    /// Describe what events you want to ignore.
    ///
    /// If you don't care about System Exclusive messages
    /// (manufacturer specific messages to their proprietary devices),
    /// set this value to [`Ignore::Sysex`].
    pub ignore: Ignore,
}

impl Default for MidiSettings {
    /// Assigns client name and port name to `bevy_midix`
    ///
    /// ignore is set to [`Ignore::None`]
    fn default() -> Self {
        Self {
            client_name: "bevy_midix",
            port_name: "bevy_midix",
            ignore: Ignore::None,
        }
    }
}
