use crate::prelude::*;

/// Represents a MIDI message, usually associated to a MIDI channel.
///
/// There are seven different types of voice events.
/// - note on: Play a particular note
/// - note off: Stop playing a particular note
/// - [control change](Controller): Things like panning, modulation, expression, holding a pedal and the alike.
/// - [program change](Program): a way to change the underlying "voice" of a channel's instrument.
/// - after touch: After a note's been played, modify the [`Velocity`] without having
///   to send another note on command
/// - channel pressure after touch: change the velocity for all currently playing notes. this one's pretty unusual to find.
/// - [pitch bend](PitchBend): "curve" the frequency of a note.
///
/// If you wish to parse a MIDI message from a slice of raw MIDI bytes, use the
/// [`LiveEvent::parse`](live/enum.LiveEvent.html#method.parse) method instead and ignore all
/// variants except for [`LiveEvent::Midi`](live/enum.LiveEvent.html#variant.Midi).
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub enum VoiceEvent {
    /// Modify the value of a MIDI controller.
    ControlChange(Controller),

    /// Change the program (also known as instrument) for a channel.
    ProgramChange {
        /// The new program (instrument) to use for the channel.
        program: Program,
    },
    /// Stop playing a note.
    NoteOff {
        /// The MIDI key to stop playing.
        key: Key,
        /// The velocity with which to stop playing it.
        velocity: Velocity,
    },
    /// Start playing a note.
    NoteOn {
        /// The key to start playing.
        key: Key,

        /// The velocity (strength) with which to press it.
        ///
        /// Note that by convention a `NoteOn` message with a velocity of 0 is equivalent to a
        /// `NoteOff`.
        velocity: Velocity,
    },
    /// Modify the velocity of a note after it has been played.
    Aftertouch {
        /// The key for which to modify its velocity.
        key: Key,
        /// The new velocity for the key.
        velocity: Velocity,
    },

    /// Change the note velocity of a whole channel at once, without starting new notes.
    ChannelPressureAfterTouch {
        /// The new velocity for all notes currently playing in the channel.
        velocity: Velocity,
    },
    /// Set the pitch bend value for the entire channel.
    PitchBend(PitchBend),
}

impl VoiceEvent {
    /// Create a note on voice event
    pub const fn note_on(key: Key, velocity: Velocity) -> Self {
        Self::NoteOn { key, velocity }
    }
    /// Create a note off voice event
    pub const fn note_off(key: Key, velocity: Velocity) -> Self {
        Self::NoteOff { key, velocity }
    }
    /// Modify the velocity of a currently played key
    pub const fn after_touch(key: Key, velocity: Velocity) -> Self {
        Self::Aftertouch { key, velocity }
    }
    /// Modify the velocity of all currently played keys
    pub const fn channel_after_touch(velocity: Velocity) -> Self {
        Self::ChannelPressureAfterTouch { velocity }
    }
    /// Set a new instrument to use for the channel
    pub const fn program_change(program: Program) -> Self {
        Self::ProgramChange { program }
    }
    /// Adjust the "pitch" of a note
    pub const fn pitch_bend(pitch_bend: PitchBend) -> Self {
        Self::PitchBend(pitch_bend)
    }

    /// Create a new control change event with the provided controller
    pub const fn control_change(controller: Controller) -> Self {
        Self::ControlChange(controller)
    }
    /// Turn self into a ChannelVoiceMessage
    pub const fn send_to_channel(self, channel: Channel) -> ChannelVoiceMessage {
        ChannelVoiceMessage::new(channel, self)
    }

    /// Returns true if the note is on. This excludes note on where the velocity is zero.
    pub fn is_note_on(&self) -> bool {
        use VoiceEvent::*;
        match self {
            NoteOn { velocity, .. } => velocity.byte() != 0,
            _ => false,
        }
    }

    /// Returns true if the note is off. This includes note on where the velocity is zero.
    pub fn is_note_off(&self) -> bool {
        use VoiceEvent::*;
        match self {
            NoteOff { .. } => true,
            NoteOn { velocity, .. } => velocity.byte() == 0,
            _ => false,
        }
    }

    // /// Get the raw data bytes for this message
    // pub fn to_raw(&self) -> Vec<u8> {
    //     match self {
    //         VoiceEvent::NoteOff { key, velocity } => vec![key.byte(), velocity.byte()],
    //         VoiceEvent::NoteOn { key, velocity } => vec![key.byte(), velocity.byte()],
    //         VoiceEvent::Aftertouch { key, velocity } => {
    //             vec![key.byte(), velocity.byte()]
    //         }
    //         VoiceEvent::ControlChange(control) => control.to_raw(),
    //         VoiceEvent::ProgramChange { program } => vec![program.byte()],
    //         VoiceEvent::ChannelPressureAfterTouch { velocity } => vec![velocity.byte()],
    //         VoiceEvent::PitchBend(bend) => {
    //             vec![bend.lsb(), bend.msb()]
    //         }
    //     }
    // }

    /// Returns the upper four bits for the status. This should be combined with the channel to make the status byte.
    /// i.e. this will return 00001000.
    /// a channel of 00001001
    /// should make 10001001
    ///
    /// TODO
    #[allow(dead_code)]
    pub(crate) const fn status_nibble(&self) -> u8 {
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
