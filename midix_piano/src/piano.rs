use bevy::prelude::*;
use bevy_midix::prelude::*;

use crate::ui::HoverNode;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_piano);
}

#[derive(Component)]
struct Piano;

fn spawn_piano(mut commands: Commands) {
    commands
        .spawn((
            Piano,
            Node {
                // fill the entire window
                width: Val::Percent(100.),
                height: Val::Px(100.),
                row_gap: Val::Px(3.),
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Relative,
                ..Default::default()
            },
            BackgroundColor(Color::linear_rgb(1., 0., 0.)),
        ))
        .with_children(|parent| {
            for key in Key::all() {
                let background_color = if key.is_sharp() {
                    Color::BLACK
                } else {
                    Color::WHITE
                };
                parent
                    .spawn((
                        key,
                        Node {
                            width: Val::Px(20.),
                            height: Val::Percent(100.),
                            ..Default::default()
                        },
                        Outline {
                            width: Val::Px(2.),
                            offset: Val::Px(0.),
                            color: Color::BLACK,
                        },
                        BorderColor(Color::BLACK),
                        BackgroundColor(background_color),
                    ))
                    .observe(on_mouse_down)
                    .observe(on_mouse_up)
                    .observe(on_mouse_enter);
            }
        });
}

fn on_mouse_enter(
    trigger: Trigger<Pointer<Over>>,
    keys: Query<&Key<'static>>,
    mut hover_node: Query<&mut Text, With<HoverNode>>,
) {
    let key = keys.get(trigger.entity()).unwrap();
    let mut hover_node = hover_node.get_single_mut().unwrap();
    hover_node.0 = format!("Last Hovered Key: {}", key);
}

fn on_mouse_down(trigger: Trigger<Pointer<Down>>, keys: Query<&Key<'static>>) {
    let triggered_entity = trigger.entity();

    let clicked_key = keys.get(triggered_entity).unwrap();

    println!("clicked {}", clicked_key);
}

fn on_mouse_up(trigger: Trigger<Pointer<Up>>, keys: Query<&Key<'static>>) {
    let triggered_entity = trigger.entity();

    let clicked_key = keys.get(triggered_entity).unwrap();

    println!("unclicked {}", clicked_key);
}
