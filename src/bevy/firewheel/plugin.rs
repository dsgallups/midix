use crate::bevy::asset::SoundFont;
use crate::bevy::firewheel::components::*;
use crate::bevy::firewheel::node::{MidiNodeEvent, MidiSynthNodeConfig, MidiSynthProcessor};
use bevy::prelude::*;
use bevy_seedling::prelude::*;
use firewheel::event::NodeEvent;

/// Plugin for MIDI synthesis using Firewheel/bevy_seedling
pub struct FirewheelMidiPlugin;

impl Plugin for FirewheelMidiPlugin {
    fn build(&self, app: &mut App) {
        // Register our custom node type with bevy_seedling
        app.register_node::<MidiSynthNodeConfig, MidiSynthProcessor>();

        // Initialize soundfont assets
        #[cfg(feature = "std")]
        {
            app.init_asset::<SoundFont>()
                .init_asset_loader::<crate::bevy::asset::SoundFontLoader>();
        }

        // Add our systems
        app.add_systems(
            Update,
            (spawn_midi_nodes, process_midi_commands)
                .chain()
                .run_if(resource_exists::<AudioContext>),
        );
    }
}

/// System that spawns MIDI synthesizer nodes for entities with soundfonts
fn spawn_midi_nodes(
    mut commands: Commands,
    audio_context: Res<AudioContext>,
    soundfont_assets: Res<Assets<SoundFont>>,
    query: Query<
        (Entity, &MidiSoundfont, Option<&MidiSynthConfig>),
        (Without<MidiSynthNode>, With<MidiCommands>),
    >,
) {
    for (entity, soundfont, config) in &query {
        // Check if soundfont is loaded
        let Some(soundfont_asset) = soundfont_assets.get(&soundfont.handle) else {
            continue;
        };

        // Get config or use defaults
        let config = config.cloned().unwrap_or_default();

        // Create node configuration
        let node_config = MidiSynthNodeConfig {
            soundfont: soundfont_asset.file.clone(),
            sample_rate: 44100.0, // Default, will be overridden by audio context
            enable_reverb: config.enable_reverb,
            enable_chorus: config.enable_chorus,
            volume: config.volume,
        };

        // Spawn the audio node
        let node = audio_context.spawn_node(node_config);

        // Connect to main output
        audio_context.connect(&node, &MainBus);

        // Add the node component to the entity
        commands.entity(entity).insert(MidiSynthNode { node });
    }
}

/// System that processes MIDI commands and sends them to the audio nodes
fn process_midi_commands(
    mut events: EventWriter<NodeEventOf<MidiNodeEvent>>,
    mut query: Query<(&MidiSynthNode, &mut MidiCommands)>,
) {
    for (synth_node, mut commands) in &mut query {
        if commands.queue.is_empty() {
            continue;
        }

        // Take all pending commands
        let pending = commands.take();

        // Send commands to the audio node as events
        for command in pending {
            events.send(NodeEventOf {
                node: synth_node.node,
                event: MidiNodeEvent { command },
            });
        }
    }
}

/// Helper event type for sending events to specific nodes
#[derive(Event)]
struct NodeEventOf<T: NodeEvent> {
    node: FirewheelNode,
    event: T,
}

/// Extension trait for Commands to easily spawn MIDI synths
pub trait MidiCommandsExt {
    /// Spawn a MIDI synthesizer with the given soundfont
    fn spawn_midi_synth(&mut self, soundfont: Handle<SoundFont>) -> Entity;

    /// Spawn a MIDI synthesizer with custom configuration
    fn spawn_midi_synth_with_config(
        &mut self,
        soundfont: Handle<SoundFont>,
        config: MidiSynthConfig,
    ) -> Entity;
}

impl MidiCommandsExt for Commands<'_, '_> {
    fn spawn_midi_synth(&mut self, soundfont: Handle<SoundFont>) -> Entity {
        self.spawn((
            MidiSoundfont { handle: soundfont },
            MidiCommands::default(),
            MidiSynthConfig::default(),
            Name::new("MIDI Synthesizer"),
        ))
        .id()
    }

    fn spawn_midi_synth_with_config(
        &mut self,
        soundfont: Handle<SoundFont>,
        config: MidiSynthConfig,
    ) -> Entity {
        self.spawn((
            MidiSoundfont { handle: soundfont },
            MidiCommands::default(),
            config,
            Name::new("MIDI Synthesizer"),
        ))
        .id()
    }
}
