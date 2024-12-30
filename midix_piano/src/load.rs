use bevy::prelude::*;
pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);

    //commands.spawn()
}
