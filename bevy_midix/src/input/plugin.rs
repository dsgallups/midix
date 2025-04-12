use super::{MidiInput, MidiInputSettings};
use bevy::prelude::*;

#[doc = r#"
Inserts [`MidiInput`] as a resource.

See [`MidiInputSettings`] for configuration options.
"#]
#[derive(Clone, Copy, Debug, Default)]
pub struct MidiInputPlugin {
    /// The settings to apply to [`MidiInput`] on instantiation.
    pub settings: MidiInputSettings,
}

impl Plugin for MidiInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MidiInput::new(self.settings));
    }
}
