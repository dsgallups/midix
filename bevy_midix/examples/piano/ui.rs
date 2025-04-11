use bevy::{
    color::palettes::css::{GREEN, RED},
    prelude::*,
};
use bevy_midix::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_piano)
        .add_systems(Update, interaction);
}

#[derive(Component)]
pub struct Piano;

fn spawn_piano(mut commands: Commands) {
    commands.spawn(Camera2d);
    warn!("Spawning piano");

    let get_note = |i: u8| {
        use Note::*;
        match i % 12 {
            0 => A,
            1 => ASharp,
            2 => B,
            3 => C,
            4 => CSharp,
            5 => D,
            6 => DSharp,
            7 => E,
            8 => F,
            9 => FSharp,
            10 => G,
            _ => GSharp,
        }
    };
    let get_octave = |i: u8| {
        // piano octave starts at 2.
        let octave = i / 12;
        Octave::new(octave as i8)
    };

    commands
        .spawn((
            Piano,
            Node {
                padding: UiRect::top(Val::Px(5.))
                    .with_right(Val::Px(5.))
                    .with_left(Val::Px(5.)),
                box_sizing: BoxSizing::BorderBox,
                width: Val::Percent(100.),
                height: Val::Px(80.),
                ..default()
            },
        ))
        .with_children(|commands| {
            (0..88).for_each(|i| {
                let note = get_note(i);
                let octave = get_octave(i);
                let key = Key::new(note, octave);

                commands.spawn((
                    Button,
                    Node {
                        flex_grow: 1.,
                        border: UiRect::all(Val::Px(1.)),
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BackgroundColor(bg_color(key.is_sharp())),
                    key,
                ));
            })
        });
}

fn bg_color(sharp: bool) -> Color {
    if sharp {
        Color::BLACK
    } else {
        Color::WHITE
    }
}

fn interaction(
    mut interactions: Query<(&Interaction, &mut BackgroundColor, &Key), Changed<Interaction>>,
    mut synth: ResMut<Synth>,
) {
    for (interaction, mut background_color, key) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
                warn!("{key} pressed");
                *background_color = RED.into();
                let event =
                    VoiceEvent::note_on(*key, Velocity::max()).send_to_channel(Channel::One);
                synth.handle_event(event);
            }
            Interaction::Hovered => {
                warn!("{key} hovered");
                *background_color = GREEN.into();
            }
            Interaction::None => {
                *background_color = BackgroundColor(bg_color(key.is_sharp()));

                let event =
                    VoiceEvent::note_on(*key, Velocity::max()).send_to_channel(Channel::One);
                synth.handle_event(event);
            }
        }
    }
}
