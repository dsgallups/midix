use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_hover_element_ui);
}

#[derive(Component)]
pub struct HoverNode;

fn spawn_hover_element_ui(mut commands: Commands) {
    commands.spawn((HoverNode, Text::new("Hover a Key"), Node {
        position_type: PositionType::Absolute,
        bottom: Val::Px(12.0),
        right: Val::Px(12.0),
        ..default()
    }));
}
