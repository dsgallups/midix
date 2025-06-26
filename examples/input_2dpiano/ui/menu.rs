use bevy::{
    color::palettes::{css::GREEN, tailwind::YELLOW_200},
    prelude::*,
};
use midix::prelude::*;
#[derive(Component)]
pub struct MenuText;

pub fn spawn_connect_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn((
            Text::default(),
            TextFont {
                font: font.clone(),
                font_size: 30.,
                ..default()
            },
            MenuText,
        ))
        .with_children(|commands| {
            commands.spawn((
                TextSpan::new(
                    "INSTRUCTIONS\n\
                    R - Refresh ports\n\
                    0 to 9 - Connect to port\n\
                    Escape - Disconnect from current port\n\n",
                ),
                TextFont {
                    font: font.clone(),
                    font_size: 30.,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
            commands.spawn((
                TextSpan::new("Available Ports: \n"),
                TextFont {
                    font: font.clone(),
                    font_size: 30.,
                    ..Default::default()
                },
                TextColor(YELLOW_200.into()),
            ));
            commands.spawn((
                TextSpan::default(),
                TextFont {
                    font: font.clone(),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(GREEN.into()),
                AvailablePortsText,
            ));
        });
}

#[derive(Component)]
pub struct AvailablePortsText;

pub fn update_available_ports(
    input: Res<MidiInput>,
    mut instructions: Query<&mut TextSpan, With<AvailablePortsText>>,
) {
    if input.is_changed() {
        let mut instructions = instructions.single_mut().unwrap();
        let mut value = String::new();

        for (i, port) in input.ports().iter().enumerate() {
            value.push_str(&format!("Port {}: {:?}\n", i, port.id()));
        }
        value.push('\n');

        instructions.0 = value;
    }
}

pub fn cleanup(mut commands: Commands, menu: Query<Entity, With<MenuText>>) {
    let menu = menu.single().unwrap();
    commands.entity(menu).despawn();
}
