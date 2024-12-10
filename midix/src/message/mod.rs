use crate::{num::u7, prelude::*, Key, PitchBend};

/// Represents a MIDI message, usually associated to a MIDI channel.
///
/// If you wish to parse a MIDI message from a slice of raw MIDI bytes, use the
/// [`LiveEvent::parse`](live/enum.LiveEvent.html#method.parse) method instead and ignore all
/// variants except for [`LiveEvent::Midi`](live/enum.LiveEvent.html#variant.Midi).
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum MidiMessage {
    /// Stop playing a note.
    NoteOff {
        /// The MIDI key to stop playing.
        key: Key,
        /// The velocity with which to stop playing it.
        vel: u7,
    },
    /// Start playing a note.
    NoteOn {
        /// The key to start playing.
        key: Key,
        /// The velocity (strength) with which to press it.
        ///
        /// Note that by convention a `NoteOn` message with a velocity of 0 is equivalent to a
        /// `NoteOff`.
        vel: u7,
    },
    /// Modify the velocity of a note after it has been played.
    Aftertouch {
        /// The key for which to modify its velocity.
        key: u7,
        /// The new velocity for the key.
        vel: u7,
    },
    /// Modify the value of a MIDI controller.
    Controller {
        /// The controller to modify.
        ///
        /// See the MIDI spec for the meaning of each index.
        controller: u7,
        /// The value to set it to.
        value: u7,
    },
    /// Change the program (also known as instrument) for a channel.
    ProgramChange {
        /// The new program (instrument) to use for the channel.
        program: u7,
    },
    /// Change the note velocity of a whole channel at once, without starting new notes.
    ChannelAftertouch {
        /// The new velocity for all notes currently playing in the channel.
        vel: u7,
    },
    /// Set the pitch bend value for the entire channel.
    PitchBend(PitchBend),
}
impl MidiMessage {
    /// Midi messages have a known length.
    pub(crate) fn msg_length(status: u8) -> usize {
        const LENGTH_BY_STATUS: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 1, 1, 2, 0];
        LENGTH_BY_STATUS[(status >> 4) as usize] as usize
    }

    /// Extract the data bytes from a raw slice.
    pub(crate) fn read_data_u8(status: u8, raw: &mut &[u8]) -> Result<[u7; 2]> {
        let len = Self::msg_length(status);
        let data = raw
            .split_checked(len)
            .ok_or_else(|| err_invalid!("truncated midi message"))?;
        Ok(match len {
            1 => [u7::check_int(data[0])?, u7::from(0)],
            2 => [u7::check_int(data[0])?, u7::check_int(data[1])?],
            _ => [u7::from(0), u7::from(0)],
        })
    }

    /// Get the data bytes from a databyte slice.
    pub(crate) fn get_data_u7(status: u8, data: &[u7]) -> Result<[u7; 2]> {
        let len = Self::msg_length(status);
        ensure!(data.len() >= len, err_invalid!("truncated midi message"));
        Ok(match len {
            1 => [data[0], u7::from(0)],
            2 => [data[0], data[1]],
            _ => [u7::from(0), u7::from(0)],
        })
    }

    /// Receives status byte and midi args separately.
    ///
    /// Panics if the `status` is not a MIDI message status (0x80..=0xEF).
    pub(crate) fn read(status: u8, data: [u7; 2]) -> (u4, MidiMessage) {
        let channel = u4::from(status);
        let msg = match status >> 4 {
            0x8 => MidiMessage::NoteOff {
                key: Key::new(data[0]),
                vel: data[1],
            },
            0x9 => MidiMessage::NoteOn {
                key: Key::new(data[0]),
                vel: data[1],
            },
            0xA => MidiMessage::Aftertouch {
                key: data[0],
                vel: data[1],
            },
            0xB => MidiMessage::Controller {
                controller: data[0],
                value: data[1],
            },
            0xC => MidiMessage::ProgramChange { program: data[0] },
            0xD => MidiMessage::ChannelAftertouch { vel: data[0] },
            0xE => {
                //Note the little-endian order, contrasting with the default big-endian order of
                //Standard Midi Files
                let lsb = data[0].as_int() as u16;
                let msb = data[1].as_int() as u16;
                MidiMessage::PitchBend(PitchBend::new(u14::from(msb << 7 | lsb)))
            }
            _ => panic!("parsed midi message before checking that status is in range"),
        };
        (channel, msg)
    }
    /// Get the raw status nibble for this MIDI message type.
    pub(crate) fn status_nibble(&self) -> u8 {
        match self {
            MidiMessage::NoteOff { .. } => 0x8,
            MidiMessage::NoteOn { .. } => 0x9,
            MidiMessage::Aftertouch { .. } => 0xA,
            MidiMessage::Controller { .. } => 0xB,
            MidiMessage::ProgramChange { .. } => 0xC,
            MidiMessage::ChannelAftertouch { .. } => 0xD,
            MidiMessage::PitchBend { .. } => 0xE,
        }
    }
    /// Write the data part of this message, not including the status.
    pub(crate) fn write<W: Write>(&self, out: &mut W) -> WriteResult<W> {
        match self {
            MidiMessage::NoteOff { key, vel } => out.write(&[key.as_int(), vel.as_int()])?,
            MidiMessage::NoteOn { key, vel } => out.write(&[key.as_int(), vel.as_int()])?,
            MidiMessage::Aftertouch { key, vel } => out.write(&[key.as_int(), vel.as_int()])?,
            MidiMessage::Controller { controller, value } => {
                out.write(&[controller.as_int(), value.as_int()])?
            }
            MidiMessage::ProgramChange { program } => out.write(&[program.as_int()])?,
            MidiMessage::ChannelAftertouch { vel } => out.write(&[vel.as_int()])?,
            MidiMessage::PitchBend(bend) => {
                let raw = bend.as_u16();
                out.write(&[(raw & 0x7F) as u8, (raw >> 7) as u8])?
            }
        }
        Ok(())
    }
}
