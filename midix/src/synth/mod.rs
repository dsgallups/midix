#![doc = r#"
Extensions for `rustysynth` interop
"#]
use rustysynth::*;

#[doc = r#"
Allows a synthesizer to communicate with [`MidiMessage`]s.
"#]
pub trait MidiSynth {}

impl MidiSynth for Synthesizer {}

#[test]
fn test_synth() {
    use std::fs::File;
    use std::sync::Arc;

    // Load the SoundFont.
    let mut sf2 = File::open("TimGM6mb.sf2").unwrap();
    let sound_font = Arc::new(SoundFont::new(&mut sf2).unwrap());

    // Load the MIDI file.
    let mut mid = File::open("flourish.mid").unwrap();
    let midi_file = Arc::new(MidiFile::new(&mut mid).unwrap());

    // Create the MIDI file sequencer.
    let settings = SynthesizerSettings::new(44100);
    let synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();
    let mut sequencer = MidiFileSequencer::new(synthesizer);

    // Play the MIDI file.
    sequencer.play(&midi_file, false);

    // The output buffer.
    let sample_count = (settings.sample_rate as f64 * midi_file.get_length()) as usize;
    let mut left: Vec<f32> = vec![0_f32; sample_count];
    let mut right: Vec<f32> = vec![0_f32; sample_count];

    // Render the waveform.
    sequencer.render(&mut left[..], &mut right[..]);
}
