use bevy::{
    log::{Level, LogPlugin},
    pbr::AmbientLight,
    prelude::*,
};
use bevy_midix::prelude::{Key as MidiKey, *};

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::WARN,
            filter: "bevy_midi=debug".to_string(),
            ..default()
        }))
        .add_plugins(MidiInputPlugin)
        .init_resource::<MidiInputSettings>()
        .add_plugins(MidiOutputPlugin)
        .init_resource::<MidiOutputSettings>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_midi_input,
                connect_to_first_input_port,
                connect_to_first_output_port,
                display_press,
                display_release,
            ),
        )
        .run();
}

#[derive(Component, Debug)]
struct Key {
    key_val: MidiKey,
    y_reset: f32,
}

#[derive(Component)]
struct PressedKey;

#[rustfmt::skip]
fn setup(
    mut cmds: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mid = -6.3;

    // light
    cmds.spawn((
        PointLight::default(),
        Transform::from_xyz(0.0, 6.0, mid)
    ));

    //Camera
    cmds.spawn((
        Camera3d::default(),
        Msaa::Sample4,
        Transform::from_xyz(8., 5., mid).looking_at(Vec3::new(0., 0., mid), Vec3::Y)
    ));

    let pos: Vec3 = Vec3::new(0., 0., 0.);

    let mut black_key: Handle<Mesh> = asset_server.load("models/black_key.gltf#Mesh0/Primitive0");
    let mut white_key_0: Handle<Mesh> = asset_server.load("models/white_key_0.gltf#Mesh0/Primitive0");
    let mut white_key_1: Handle<Mesh> = asset_server.load("models/white_key_1.gltf#Mesh0/Primitive0");
    let mut white_key_2: Handle<Mesh> = asset_server.load("models/white_key_2.gltf#Mesh0/Primitive0");
    let b_mat = materials.add(Color::srgb(0.1, 0.1, 0.1));
    let w_mat = materials.add(Color::srgb(1.0, 1.0, 1.0));

    //Create keyboard layout
    let pos_black = pos + Vec3::new(0., 0.06, 0.);

    for i in (0..8).map(Octave::new) {

        spawn_note(&mut cmds, &w_mat, 0.00, pos, &mut white_key_0, i, Note::C);
        spawn_note(&mut cmds, &b_mat, 0.15, pos_black, &mut black_key, i, Note::CSharp);
        spawn_note(&mut cmds, &w_mat, 0.27, pos, &mut white_key_1, i, Note::D);
        spawn_note(&mut cmds, &b_mat, 0.39, pos_black, &mut black_key, i, Note::DSharp);
        spawn_note(&mut cmds, &w_mat, 0.54, pos, &mut white_key_2, i, Note::E);
        spawn_note(&mut cmds, &w_mat, 0.69, pos, &mut white_key_0, i, Note::F);
        spawn_note(&mut cmds, &b_mat, 0.85, pos_black, &mut black_key, i, Note::FSharp);
        spawn_note(&mut cmds, &w_mat, 0.96, pos, &mut white_key_1, i, Note::G);
        spawn_note(&mut cmds, &b_mat, 1.08, pos_black, &mut black_key, i, Note::GSharp);
        spawn_note(&mut cmds, &w_mat, 1.19, pos, &mut white_key_1, i, Note::A);
        spawn_note(&mut cmds, &b_mat, 1.31, pos_black, &mut black_key, i, Note::ASharp);
        spawn_note(&mut cmds, &w_mat, 1.46, pos, &mut white_key_2, i, Note::B);
    }
}

fn spawn_note(
    commands: &mut Commands,
    mat: &Handle<StandardMaterial>,
    offset_z: f32,
    pos: Vec3,
    asset: &mut Handle<Mesh>,
    octave: Octave,
    note: Note,
) {
    commands.spawn((
        Mesh3d(asset.clone()),
        MeshMaterial3d(mat.clone()),
        Transform {
            translation: Vec3::new(
                pos.x,
                pos.y,
                pos.z - offset_z - (1.61 * octave.value() as f32),
            ),
            scale: Vec3::new(10., 10., 10.),
            ..Default::default()
        },
        Key {
            key_val: MidiKey::from_note_and_octave(note, octave), //TODO
            y_reset: pos.y,
        },
    ));
}

fn display_press(mut query: Query<&mut Transform, With<PressedKey>>) {
    for mut t in &mut query {
        t.translation.y = -0.05;
    }
}

fn display_release(mut query: Query<(&mut Transform, &Key), Without<PressedKey>>) {
    for (mut t, k) in &mut query {
        t.translation.y = k.y_reset;
    }
}

fn handle_midi_input(
    mut commands: Commands,
    mut midi_events: EventReader<MidiData>,
    query: Query<(Entity, &Key)>,
) {
    for data in midi_events.read() {
        let raw = data.message.to_bytes();
        let [_, index, _value] = raw.as_slice() else {
            continue;
        };
        let midi_key = MidiKey::new(index).unwrap();

        if let LiveEvent::ChannelVoice(message) = &data.message {
            if message.is_note_on() {
                for (entity, key) in query.iter() {
                    if key.key_val.eq(&midi_key) {
                        commands.entity(entity).insert(PressedKey);
                    }
                }
            } else if message.is_note_off() {
                for (entity, key) in query.iter() {
                    if key.key_val.eq(&midi_key) {
                        commands.entity(entity).remove::<PressedKey>();
                    }
                }
            }
        }
    }
}

fn connect_to_first_input_port(input: Res<MidiInput>) {
    if input.is_changed() {
        if let Some((_, port)) = input.ports().first() {
            input.connect(port.clone());
        }
    }
}

fn connect_to_first_output_port(input: Res<MidiOutput>) {
    if input.is_changed() {
        if let Some((_, port)) = input.ports().first() {
            input.connect(port.clone());
        }
    }
}
