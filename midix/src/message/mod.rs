use controller::Controller;
use key::Key;
use pitch_bend::PitchBend;
use program::Program;
use velocity::Velocity;

use crate::{
    bytes::{FromMidiMessage, MidiBits, ReadDataBytes},
    utils::check_u7,
    Channel,
};
pub mod controller;
pub mod key;
pub mod pitch_bend;
pub mod program;
pub mod velocity;

/// Represents a MIDI message, usually associated to a MIDI channel.
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
            0xB => ChannelVoiceEvent::Controller {
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
                let lsb = data[0].as_int() as u16;
                let msb = data[1].as_int() as u16;
                ChannelVoiceEvent::PitchBend {
                    bend: PitchBend(u14::from(msb << 7 | lsb)),
                }
            }
            _ => panic!("parsed midi message before checking that status is in range"),
        };
        todo!()
    }
}

impl ChannelVoiceMessage {
    const MIN_STATUS_BYTE: u8 = 0x80;
    const MAX_STATUS_BYTE: u8 = 0xEF;

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

    /// Get the raw midi packet for this message
    pub fn to_raw(&self) -> Vec<u8> {
        let raw_status = self.status_nibble();

        match self.message {
            ChannelVoiceEvent::NoteOff { key, vel } => vec![raw_status, key.as_int(), vel.as_int()],
            ChannelVoiceEvent::NoteOn { key, vel } => vec![raw_status, key.as_int(), vel.as_int()],
            ChannelVoiceEvent::Aftertouch { key, vel } => {
                vec![raw_status, key.as_int(), vel.as_int()]
            }
            ChannelVoiceEvent::Controller { controller, value } => {
                vec![raw_status, controller, value]
            }
            ChannelVoiceEvent::ProgramChange { program } => vec![raw_status, program],
            ChannelVoiceEvent::ChannelPressureAfterTouch { vel } => vec![raw_status, vel],
            ChannelVoiceEvent::PitchBend(bend) => {
                let raw = bend.as_u16();
                vec![raw_status, (raw & 0x7F) as u8, (raw >> 7) as u8]
            }
        }
    }

    /// Get the raw status nibble for this MIDI message type.
    pub(crate) fn status_nibble(&self) -> u8 {
        self.message.status_nibble()
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
    Controller {
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
/// Midi messages have a known length.
pub(crate) fn msg_length(status: u8) -> usize {
    const LENGTH_BY_STATUS: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 1, 1, 2, 0];
    LENGTH_BY_STATUS[(status >> 4) as usize] as usize
}

/// Receives status byte and midi args separately.
///
/// Panics if the `status` is not a MIDI message status (0x80..=0xEF).
pub(crate) fn read(status: u8, data: [u8; 2]) -> (u8, ChannelVoiceEvent) {
    let msg = match status >> 4 {
        0x8 => ChannelVoiceEvent::NoteOff {
            key: Key::new(data[0]),
            vel: Velocity::new(data[1]),
        },
        0x9 => ChannelVoiceEvent::NoteOn {
            key: Key::new(data[0]),
            vel: Velocity::new(data[1]),
        },
        0xA => ChannelVoiceEvent::Aftertouch {
            key: Key::new(data[0]),
            vel: Velocity::new(data[1]),
        },
        0xB => ChannelVoiceEvent::Controller {
            controller: data[0],
            value: data[1],
        },
        0xC => ChannelVoiceEvent::ProgramChange { program: data[0] },
        0xD => ChannelVoiceEvent::ChannelPressureAfterTouch { vel: data[0] },
        0xE => {
            //Note the little-endian order, contrasting with the default big-endian order of
            //Standard Midi Files
            let lsb = data[0] as u16;
            let msb = data[1] as u16;
            ChannelVoiceEvent::PitchBend(PitchBend::new(msb << 7 | lsb))
        }
        _ => panic!("parsed midi message before checking that status is in range"),
    };
    (status, msg)
}

impl ChannelVoiceEvent {
    /// Returns true if the note is on. This excludes note on where the velocity is zero.
    pub fn is_note_on(&self) -> bool {
        use ChannelVoiceEvent::*;
        match self {
            NoteOn { vel, .. } => vel.as_int() != 0,
            _ => false,
        }
    }

    /// Returns true if the note is off. This includes note on where the velocity is zero.
    pub fn is_note_off(&self) -> bool {
        use ChannelVoiceEvent::*;
        match self {
            NoteOff { .. } => true,
            NoteOn { vel, .. } => vel.as_int() == 0,
            _ => false,
        }
    }

    /// Get the raw midi packet for this message
    pub fn to_raw(&self) -> Vec<u8> {
        let raw_status = self.status_nibble();

        match self {
            ChannelVoiceEvent::NoteOff { key, vel } => vec![raw_status, key.as_int(), vel.as_int()],
            ChannelVoiceEvent::NoteOn { key, vel } => vec![raw_status, key.as_int(), vel.as_int()],
            ChannelVoiceEvent::Aftertouch { key, vel } => {
                vec![raw_status, key.as_int(), vel.as_int()]
            }
            ChannelVoiceEvent::Controller { controller, value } => {
                vec![raw_status, *controller, *value]
            }
            ChannelVoiceEvent::ProgramChange { program } => vec![raw_status, *program],
            ChannelVoiceEvent::ChannelPressureAfterTouch { vel } => vec![raw_status, *vel],
            ChannelVoiceEvent::PitchBend(bend) => {
                let raw = bend.as_u16();
                vec![raw_status, (raw & 0x7F) as u8, (raw >> 7) as u8]
            }
        }
    }

    /// Get the raw status nibble for this MIDI message type.
    pub(crate) fn status_nibble(&self) -> u8 {
        match self {
            ChannelVoiceEvent::NoteOff { .. } => 0x8,
            ChannelVoiceEvent::NoteOn { .. } => 0x9,
            ChannelVoiceEvent::Aftertouch { .. } => 0xA,
            ChannelVoiceEvent::Controller { .. } => 0xB,
            ChannelVoiceEvent::ProgramChange { .. } => 0xC,
            ChannelVoiceEvent::ChannelPressureAfterTouch { .. } => 0xD,
            ChannelVoiceEvent::PitchBend { .. } => 0xE,
        }
    }
}
