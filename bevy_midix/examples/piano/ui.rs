use bevy::{
    color::palettes::css::{GREEN, RED},
    prelude::*,
};
use bevy_midix::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_piano);
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
        //
        // note that octaves start with C, not A. so that's why we add 9 here.
        let octave = (i + 9) / 12;
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

                commands
                    .spawn((
                        Button,
                        Node {
                            flex_grow: 1.,
                            border: UiRect::all(Val::Px(1.)),
                            ..default()
                        },
                        BorderColor(Color::BLACK),
                        BackgroundColor(bg_color(key.is_sharp())),
                        key,
                    ))
                    .observe(on_mouse_enter)
                    .observe(on_mouse_down)
                    .observe(on_mouse_up)
                    .observe(on_mouse_leave);
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

const HOVERED: Srgba = GREEN;
const PRESSED: Srgba = RED;

fn on_mouse_enter(trigger: Trigger<Pointer<Over>>, mut keys: Query<(&mut BackgroundColor, &Key)>) {
    let (mut background_color, key) = keys.get_mut(trigger.target()).unwrap();
    warn!("{key} hovered");
    *background_color = HOVERED.into();
}

fn on_mouse_down(
    trigger: Trigger<Pointer<Pressed>>,
    mut keys: Query<(&mut BackgroundColor, &Key)>,
    mut synth: ResMut<Synth>,
) {
    let (mut background_color, key) = keys.get_mut(trigger.target()).unwrap();
    warn!("{key} pressed");
    *background_color = PRESSED.into();
    let event = VoiceEvent::note_on(*key, Velocity::max()).send_to_channel(Channel::One);
    synth.handle_event(event);
}
fn on_mouse_up(
    trigger: Trigger<Pointer<Released>>,
    mut keys: Query<(&mut BackgroundColor, &Key)>,
    mut synth: ResMut<Synth>,
) {
    let (mut background_color, key) = keys.get_mut(trigger.target()).unwrap();
    warn!("{key} unpressed");

    let event = VoiceEvent::note_on(*key, Velocity::zero()).send_to_channel(Channel::One);
    // could make this beter and revert to hover, but lazy
    *background_color = BackgroundColor(bg_color(key.is_sharp()));
    synth.handle_event(event);
}
fn on_mouse_leave(trigger: Trigger<Pointer<Out>>, mut keys: Query<(&mut BackgroundColor, &Key)>) {
    let (mut background_color, key) = keys.get_mut(trigger.target()).unwrap();
    *background_color = BackgroundColor(bg_color(key.is_sharp()));
}
