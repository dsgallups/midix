use bevy::prelude::*;
use bevy_midix::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_piano);
}

#[derive(Component)]
pub struct Piano;

fn spawn_piano(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Piano,
            Node {
                width: Val::Percent(100.),
                height: Val::Px(50.),
                ..default()
            },
        ))
        .with_children(|commands| {
            (0..88).for_each(|i| {
                let note = get_note(i);
                let octave = get_octave(i);
                let key = Key::from_note_and_octave(note, octave);

                let bg_color = if key.is_sharp() {
                    Color::BLACK
                } else {
                    Color::WHITE
                };

                commands.spawn((Node { ..default() }, BackgroundColor(bg_color)));
            })
        });
}

fn get_note(i: u8) -> Note {
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
}
fn get_octave(i: u8) -> Octave {
    let octave = (i / 12) + 2;
    Octave::new(octave as i8)
}
