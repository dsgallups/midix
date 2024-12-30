use bevy::prelude::*;
pub fn plugin(app: &mut App) {
    app.add_systems(Startup, load);
}

fn load(mut commands: Commands) {
    commands.spawn(Camera2d);

    //commands.spawn()
}
