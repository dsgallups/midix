use crate::prelude::*;
/// Represents a MIDI message, usually associated to a MIDI channel.
///
/// If you wish to parse a MIDI message from a slice of raw MIDI bytes, use the
/// [`LiveEvent::parse`](live/enum.LiveEvent.html#method.parse) method instead and ignore all
/// variants except for [`LiveEvent::Midi`](live/enum.LiveEvent.html#variant.Midi).
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum VoiceEvent<'a> {
    /// Stop playing a note.
    NoteOff {
        /// The MIDI key to stop playing.
        key: KeyRef<'a>,
        /// The velocity with which to stop playing it.
        velocity: VelocityRef<'a>,
    },
    /// Start playing a note.
    NoteOn {
        /// The key to start playing.
        key: KeyRef<'a>,

        /// The velocity (strength) with which to press it.
        ///
        /// Note that by convention a `NoteOn` message with a velocity of 0 is equivalent to a
        /// `NoteOff`.
        velocity: VelocityRef<'a>,
    },
    /// Modify the velocity of a note after it has been played.
    Aftertouch {
        /// The key for which to modify its velocity.
        key: KeyRef<'a>,
        /// The new velocity for the key.
        velocity: VelocityRef<'a>,
    },
    /// Modify the value of a MIDI controller.
    ControlChange {
        /// The controller to modify.
        ///
        /// See the MIDI spec for the meaning of each index.
        controller: ControllerRef<'a>,
        /// The value to set it to.
        value: &'a u8,
    },
    /// Change the program (also known as instrument) for a channel.
    ProgramChange {
        /// The new program (instrument) to use for the channel.
        program: ProgramRef<'a>,
    },
    /// Change the note velocity of a whole channel at once, without starting new notes.
    ChannelPressureAfterTouch {
        /// The new velocity for all notes currently playing in the channel.
        velocity: VelocityRef<'a>,
    },
    /// Set the pitch bend value for the entire channel.
    PitchBend(PitchBendRef<'a>),
}

impl VoiceEvent<'_> {
    /// Returns true if the note is on. This excludes note on where the velocity is zero.
    pub fn is_note_on(&self) -> bool {
        use VoiceEvent::*;
        match self {
            NoteOn { velocity, .. } => velocity.byte() != &0,
            _ => false,
        }
    }

    /// Returns true if the note is off. This includes note on where the velocity is zero.
    pub fn is_note_off(&self) -> bool {
        use VoiceEvent::*;
        match self {
            NoteOff { .. } => true,
            NoteOn { velocity, .. } => velocity.byte() == &0,
            _ => false,
        }
    }

    /*/// Get the raw data bytes for this message
    pub fn to_raw(&self) -> Vec<u8> {
        match self {
            ChannelVoiceEvent::NoteOff { key, vel } => vec![key.as_bits(), vel.as_bits()],
            ChannelVoiceEvent::NoteOn { key, vel } => vec![key.as_bits(), vel.as_bits()],
            ChannelVoiceEvent::Aftertouch { key, vel } => {
                vec![key.as_bits(), vel.as_bits()]
            }
            ChannelVoiceEvent::ControlChange { controller, value } => {
                vec![controller.as_bits(), *value]
            }
            ChannelVoiceEvent::ProgramChange { program } => vec![program.as_bits()],
            ChannelVoiceEvent::ChannelPressureAfterTouch { vel } => vec![vel.as_bits()],
            ChannelVoiceEvent::PitchBend(bend) => {
                vec![bend.lsb(), bend.msb()]
            }
        }
    }*/

    /// Returns the upper four bits for the status. This should be combined with the channel to make the status byte.
    /// i.e. this will return 00001000.
    /// a channel of 00001001
    /// should make 10001001
    ///
    /// TODO
    #[allow(dead_code)]
    pub(crate) fn status_nibble(&self) -> u8 {
        match self {
            VoiceEvent::NoteOff { .. } => 0x8,
            VoiceEvent::NoteOn { .. } => 0x9,
            VoiceEvent::Aftertouch { .. } => 0xA,
            VoiceEvent::ControlChange { .. } => 0xB,
            VoiceEvent::ProgramChange { .. } => 0xC,
            VoiceEvent::ChannelPressureAfterTouch { .. } => 0xD,
            VoiceEvent::PitchBend { .. } => 0xE,
        }
    }
}
