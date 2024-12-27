use crate::prelude::*;
mod event;
pub use event::*;

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
    /// TODO: read functions should take in an iterator that yields u8s
    pub fn read(reader: &mut Reader<&[u8]>) -> ReadResult<Self> {
        let status = reader.read_next()?;
        println!("status: {:?}", status);
        let msg = match status >> 4 {
            0x8 => ChannelVoiceEvent::NoteOff {
                key: Key::from_bits(*reader.read_next()?)?,
                vel: Velocity::from_bits(*reader.read_next()?)?,
            },
            0x9 => ChannelVoiceEvent::NoteOn {
                key: Key::from_bits(*reader.read_next()?)?,
                vel: Velocity::from_bits(*reader.read_next()?)?,
            },
            0xA => ChannelVoiceEvent::Aftertouch {
                key: Key::from_bits(*reader.read_next()?)?,
                vel: Velocity::from_bits(*reader.read_next()?)?,
            },
            0xB => ChannelVoiceEvent::ControlChange {
                controller: Controller::from_bits(*reader.read_next()?)?,
                value: check_u7(*reader.read_next()?)?,
            },
            0xC => ChannelVoiceEvent::ProgramChange {
                program: Program::from_bits(*reader.read_next()?)?,
            },
            0xD => ChannelVoiceEvent::ChannelPressureAfterTouch {
                vel: Velocity::from_bits(*reader.read_next()?)?,
            },
            0xE => {
                //Note the little-endian order, contrasting with the default big-endian order of
                //Standard Midi Files
                let lsb = *reader.read_next()?;
                let msb = *reader.read_next()?;
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

    pub fn new(channel: Channel, message: ChannelVoiceEvent) -> Self {
        Self { channel, message }
    }

    /// Returns true if the note is on. This excludes note on where the velocity is zero.
    pub fn is_note_on(&self) -> bool {
        self.message.is_note_on()
    }

    /// Returns true if the note is off. This includes note on where the velocity is zero.
    pub fn is_note_off(&self) -> bool {
        self.message.is_note_off()
    }

    /// Returns the key if the event has a key
    pub fn key(&self) -> Option<Key> {
        match self.message {
            ChannelVoiceEvent::NoteOn { key, .. }
            | ChannelVoiceEvent::NoteOff { key, .. }
            | ChannelVoiceEvent::Aftertouch { key, .. } => Some(key),
            _ => None,
        }
    }
    pub fn velocity(&self) -> Option<Velocity> {
        match self.message {
            ChannelVoiceEvent::NoteOn { vel, .. }
            | ChannelVoiceEvent::NoteOff { vel, .. }
            | ChannelVoiceEvent::Aftertouch { vel, .. }
            | ChannelVoiceEvent::ChannelPressureAfterTouch { vel } => Some(vel),
            _ => None,
        }
    }

    pub fn status(&self) -> u8 {
        self.message.status_nibble() << 4 | self.channel.bits()
    }
    pub fn message(&self) -> &ChannelVoiceEvent {
        &self.message
    }
}
impl AsMidiBytes for ChannelVoiceMessage {
    /// Get the raw midi packet for this message
    fn as_bytes(&self) -> Vec<u8> {
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
