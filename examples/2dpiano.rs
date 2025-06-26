use bevy::{
    color::palettes::css::{GREEN, RED},
    log::{Level, LogPlugin},
    prelude::*,
};
use midix::prelude::*;

///Creates a 2d Piano Keyboard and plays the sound on press.
///
/// Note: due to the size of soundfont files and the lack of optimization
/// for running this example, you should run this with example with `--release`
///
/// i.e.
/// ```console
/// cargo run --example 2dpiano --release
/// ```
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: Level::INFO,
                ..default()
            }),
            MidiPlugin {
                input: None,
                ..Default::default()
            },
        ))
        .add_systems(Startup, (load_sf2, spawn_piano))
        .add_systems(Update, handle_input)
        .run();
}
/// Take a look here for some soundfonts:
///
/// <https://sites.google.com/site/soundfonts4u/>
fn load_sf2(asset_server: Res<AssetServer>, mut synth: ResMut<Synth>) {
    synth.use_soundfont(asset_server.load("soundfont.sf2"));
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
        // piano octave starts at 0.
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
                // Note: there's an easier way to do this.
                // In theory, you can make the key by adding an amount to i such that
                // the first key is A:0 using `Key::from_databyte`.
                // Note that there are 127 available keys.
                // The first key (byte 0) is going to be C:-1. so you'd write
                //
                // `let key = Key::from_databyte(i + 9 + 12).unwrap()`
                //
                // 9 will get the next A, and 12 skips the -1 octave. most soundfonts
                // don't carry a -1 piano octave.
                //
                // However, for example purposes, I have provided the logic for getting
                // the correct key via note and octave.
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
                    .observe(on_mouse_leave)
                    .observe(on_mouse_up);
                // .observe(on_mouse_enter)
                // .observe(on_mouse_down)
                // .observe(on_mouse_up)
                // .observe(on_mouse_leave);
            })
        });
}

fn bg_color(sharp: bool) -> Color {
    if sharp { Color::BLACK } else { Color::WHITE }
}

const HOVERED: Srgba = GREEN;
const PRESSED: Srgba = RED;

// use mouse input over Interaction::Pressed so you can hold down the button and go nuts
fn handle_input(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut keys: Query<(&Interaction, &mut BackgroundColor, &Key), Changed<Interaction>>,
    synth: Res<Synth>,
) {
    for (interaction, mut background_color, key) in &mut keys {
        match *interaction {
            Interaction::Hovered | Interaction::Pressed => {
                if mouse_input.pressed(MouseButton::Left) {
                    warn!("{key} pressed");
                    *background_color = PRESSED.into();
                    let event =
                        VoiceEvent::note_on(*key, Velocity::MAX).send_to_channel(Channel::One);
                    _ = synth.push_event(event);
                } else {
                    *background_color = HOVERED.into();
                }
            }
            // on_mouse_leave does runs logic that would otherwise be here.
            Interaction::None => {}
        }
    }
}

// handles the case where you are dragging and then you release the mouse on a key.
// The Interaction component will remain Interaction::Hovered, so the handle_input system will not catch it.
fn on_mouse_up(
    trigger: Trigger<Pointer<Released>>,
    mut keys: Query<(&mut BackgroundColor, &Key)>,
    synth: Res<Synth>,
) {
    let (mut background_color, key) = keys.get_mut(trigger.target()).unwrap();
    warn!("{key} unpressed");

    let event = VoiceEvent::note_on(*key, Velocity::ZERO).send_to_channel(Channel::One);
    // could make this beter and revert to hover, but lazy
    *background_color = HOVERED.into();
    _ = synth.push_event(event);
}

// because Interaction::Pressed doesn't do anything if you leave pressed.
fn on_mouse_leave(
    trigger: Trigger<Pointer<Out>>,
    mut keys: Query<(&mut BackgroundColor, &Key)>,
    synth: Res<Synth>,
) {
    let (mut background_color, key) = keys.get_mut(trigger.target()).unwrap();
    *background_color = BackgroundColor(bg_color(key.is_sharp()));
    let event = VoiceEvent::note_on(*key, Velocity::ZERO).send_to_channel(Channel::One);
    _ = synth.push_event(event);
}
