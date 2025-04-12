#![allow(missing_docs)]
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<MidiInput>();
}
#[derive(Resource, Default)]
pub struct MidiInput {}
