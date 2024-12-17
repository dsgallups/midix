use crate::{
    bytes::{FromMidiMessage, MidiBits, ReadDataBytes},
    utils::check_u7,
    Channel,
};

use super::{
    controller::Controller, key::Key, pitch_bend::PitchBend, program::Program, velocity::Velocity,
};

/// Represents a MIDI voice message,.
///
/// If you wish to parse a MIDI message from a slice of raw MIDI bytes, use the
/// [`LiveEvent::parse`](live/enum.LiveEvent.html#method.parse) method instead and ignore all
/// variants except for [`LiveEvent::Midi`](live/enum.LiveEvent.html#variant.Midi).
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct ChannelVoiceMessage {
    /// The MIDI channel that this event is associated with.
    channel: Channel,
    /// The MIDI message type and associated data.
    message: ChannelVoiceEvent,
}

impl ChannelVoiceMessage {
    pub const MIN_STATUS_BYTE: u8 = 0x80;
    pub const MAX_STATUS_BYTE: u8 = 0xEF;

    /// Returns true if the note is on. This excludes note on where the velocity is zero.
    pub fn is_note_on(&self) -> bool {
        self.message.is_note_on()
    }

    /// Returns true if the note is off. This includes note on where the velocity is zero.
    pub fn is_note_off(&self) -> bool {
        self.message.is_note_off()
    }

    pub fn status(&self) -> u8 {
        self.message.status_nibble() << 4 | self.channel.bits()
    }
    pub fn message(&self) -> &ChannelVoiceEvent {
        &self.message
    }

    /// Get the raw midi packet for this message
    pub fn to_raw(&self) -> Vec<u8> {
        let mut packet = Vec::with_capacity(3);
        packet.push(self.status());
        let data = self.message.to_raw();
        packet.extend(data);

        packet
    }
}

impl FromMidiMessage for ChannelVoiceMessage {
    const MIN_STATUS_BYTE: u8 = 0x80;
    const MAX_STATUS_BYTE: u8 = 0xEF;
    fn from_status_and_data(status: u8, data: &[u8]) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        let msg = match status >> 4 {
            0x8 => ChannelVoiceEvent::NoteOff {
                key: Key::from_bits(*data.get_byte(0)?)?,
                vel: Velocity::from_bits(*data.get_byte(1)?)?,
            },
            0x9 => ChannelVoiceEvent::NoteOn {
                key: Key::from_bits(*data.get_byte(0)?)?,
                vel: Velocity::from_bits(*data.get_byte(1)?)?,
            },
            0xA => ChannelVoiceEvent::Aftertouch {
                key: Key::from_bits(*data.get_byte(0)?)?,
                vel: Velocity::from_bits(*data.get_byte(1)?)?,
            },
            0xB => ChannelVoiceEvent::ControlChange {
                controller: Controller::from_bits(*data.get_byte(0)?)?,
                value: check_u7(*data.get_byte(1)?)?,
            },
            0xC => ChannelVoiceEvent::ProgramChange {
                program: Program::from_bits(*data.get_byte(0)?)?,
            },
            0xD => ChannelVoiceEvent::ChannelPressureAfterTouch {
                vel: Velocity::from_bits(*data.get_byte(0)?)?,
            },
            0xE => {
                //Note the little-endian order, contrasting with the default big-endian order of
                //Standard Midi Files
                let lsb = *data.get_byte(0)?;
                let msb = *data.get_byte(1)?;
                ChannelVoiceEvent::PitchBend(PitchBend::from_byte_pair(lsb, msb)?)
            }
            _ => panic!("parsed midi message before checking that status is in range"),
        };
        let channel = status & 0b0000_1111;
        Ok(ChannelVoiceMessage {
            channel: Channel::new(channel)?,
            message: msg,
        })
    }
}

/// Represents a MIDI message, usually associated to a MIDI channel.
///
/// If you wish to parse a MIDI message from a slice of raw MIDI bytes, use the
/// [`LiveEvent::parse`](live/enum.LiveEvent.html#method.parse) method instead and ignore all
/// variants except for [`LiveEvent::Midi`](live/enum.LiveEvent.html#variant.Midi).
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum ChannelVoiceEvent {
    /// Stop playing a note.
    NoteOff {
        /// The MIDI key to stop playing.
        key: Key,
        /// The velocity with which to stop playing it.
        vel: Velocity,
    },
    /// Start playing a note.
    NoteOn {
        /// The key to start playing.
        key: Key,
        /// The velocity (strength) with which to press it.
        ///
        /// Note that by convention a `NoteOn` message with a velocity of 0 is equivalent to a
        /// `NoteOff`.
        vel: Velocity,
    },
    /// Modify the velocity of a note after it has been played.
    Aftertouch {
        /// The key for which to modify its velocity.
        key: Key,
        /// The new velocity for the key.
        vel: Velocity,
    },
    /// Modify the value of a MIDI controller.
    ControlChange {
        /// The controller to modify.
        ///
        /// See the MIDI spec for the meaning of each index.
        controller: Controller,
        /// The value to set it to.
        value: u8,
    },
    /// Change the program (also known as instrument) for a channel.
    ProgramChange {
        /// The new program (instrument) to use for the channel.
        program: Program,
    },
    /// Change the note velocity of a whole channel at once, without starting new notes.
    ChannelPressureAfterTouch {
        /// The new velocity for all notes currently playing in the channel.
        vel: Velocity,
    },
    /// Set the pitch bend value for the entire channel.
    PitchBend(PitchBend),
}

impl ChannelVoiceEvent {
    /// Returns true if the note is on. This excludes note on where the velocity is zero.
    pub fn is_note_on(&self) -> bool {
        use ChannelVoiceEvent::*;
        match self {
            NoteOn { vel, .. } => vel.as_bits() != 0,
            _ => false,
        }
    }

    /// Returns true if the note is off. This includes note on where the velocity is zero.
    pub fn is_note_off(&self) -> bool {
        use ChannelVoiceEvent::*;
        match self {
            NoteOff { .. } => true,
            NoteOn { vel, .. } => vel.as_bits() == 0,
            _ => false,
        }
    }

    /// Get the raw data bytes for this message
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
    }

    /// Returns the upper four bits for the status. This should be combined with the channel to make the status byte.
    /// i.e. this will return 00001000.
    /// a channel of 00001001
    /// should make 10001001
    pub(crate) fn status_nibble(&self) -> u8 {
        match self {
            ChannelVoiceEvent::NoteOff { .. } => 0x8,
            ChannelVoiceEvent::NoteOn { .. } => 0x9,
            ChannelVoiceEvent::Aftertouch { .. } => 0xA,
            ChannelVoiceEvent::ControlChange { .. } => 0xB,
            ChannelVoiceEvent::ProgramChange { .. } => 0xC,
            ChannelVoiceEvent::ChannelPressureAfterTouch { .. } => 0xD,
            ChannelVoiceEvent::PitchBend { .. } => 0xE,
        }
    }
}
