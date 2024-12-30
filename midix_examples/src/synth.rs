use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<Synth>();
}

#[derive(Resource)]
pub struct Synth {}

impl Default for Synth {
    fn default() -> Self {
        todo!();
    }
}
