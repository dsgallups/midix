use crate::bevy::asset::SoundFont;
use crate::bevy::firewheel::components::*;
use crate::bevy::firewheel::node::{MidiNodeEvent, MidiSynthNodeConfig, MidiSynthProcessor};
use bevy::prelude::*;
use bevy_seedling::prelude::*;

/// Plugin for MIDI synthesis using Firewheel/bevy_seedling
pub struct FirewheelMidiPlugin;

impl Plugin for FirewheelMidiPlugin {
    fn build(&self, app: &mut App) {
        // Register our custom node type with bevy_seedling
        app.register_audio_node::<MidiSynthNodeConfig>();

        // Initialize soundfont assets
        app.init_asset::<SoundFont>()
            .init_asset_loader::<crate::bevy::asset::SoundFontLoader>();

        // Add systems
        app.add_systems(
            Update,
            (spawn_midi_nodes, process_midi_commands, update_midi_config)
                .chain()
                .run_if(resource_exists::<AudioContext>),
        );
    }
}

/// System that spawns MIDI synthesizer nodes for entities with soundfonts
fn spawn_midi_nodes(
    mut commands: Commands,
    mut audio_context: ResMut<AudioContext>,
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

        // Get sample rate from audio context
        let sample_rate = audio_context.sample_rate() as f32;

        // Create node configuration
        let node_config = MidiSynthNodeConfig {
            soundfont: soundfont_asset.file.clone(),
            sample_rate,
            enable_reverb: config.enable_reverb,
            enable_chorus: config.enable_chorus,
            volume: config.volume,
        };

        // Spawn the audio node through bevy_seedling
        let node = audio_context.add_audio_node(node_config);

        // Connect the node to the main output
        audio_context.connect(&node, &AudioContext::main_output());

        // Add the node component to the entity
        commands.entity(entity).insert(MidiSynthNode { node });
    }
}

/// System that processes MIDI commands and sends them to the audio nodes
fn process_midi_commands(
    mut audio_context: ResMut<AudioContext>,
    mut query: Query<(&MidiSynthNode, &mut MidiCommands), Changed<MidiCommands>>,
) {
    for (synth_node, mut commands) in &mut query {
        if commands.queue.is_empty() {
            continue;
        }

        // Take all pending commands
        let pending = commands.take();

        // Send commands to the audio node as events
        for command in pending {
            audio_context.send_event(&synth_node.node, MidiNodeEvent { command });
        }
    }
}

/// System that updates MIDI synthesizer configuration
fn update_midi_config(
    mut audio_context: ResMut<AudioContext>,
    query: Query<(&MidiSynthNode, &MidiSynthConfig), Changed<MidiSynthConfig>>,
) {
    for (synth_node, config) in &query {
        // Update volume parameter on the node
        audio_context.set_node_param(&synth_node.node, "volume", config.volume);

        // Note: Reverb and chorus changes would require recreating the node
        // as rustysynth doesn't support changing these at runtime
    }
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

/// Helper module for system run conditions
pub mod conditions {
    use super::*;

    /// Condition that returns true when the audio context is ready
    pub fn audio_ready() -> impl Condition<()> {
        resource_exists::<AudioContext>
    }
}
