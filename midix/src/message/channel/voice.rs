use io::ErrorKind;

use crate::prelude::*;

/// Represents a MIDI voice message,.
///
/// If you wish to parse a MIDI message from a slice of raw MIDI bytes, use the
/// [`LiveEvent::parse`](live/enum.LiveEvent.html#method.parse) method instead and ignore all
/// variants except for [`LiveEvent::Midi`](live/enum.LiveEvent.html#variant.Midi).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ChannelVoiceMessage {
    /// The MIDI channel that this event is associated with.
    /// Used for getting the channel as the status' lsb contains the channel
    status: StatusByte,
    /// The MIDI message type and associated data.
    message: VoiceEvent,
}

impl ChannelVoiceMessage {
    /// Create a new channel voice event from the channel and associated event type
    pub fn new(channel: Channel, message: VoiceEvent) -> Self {
        let status = channel.to_byte() | (message.status_nibble() << 4);
        Self {
            status: StatusByte::new_unchecked(status),
            message,
        }
    }

    /// TODO: read functions should take in an iterator that yields u8s
    pub(crate) fn read<'a, R>(status: StatusByte, reader: &mut Reader<R>) -> ReadResult<Self>
    where
        R: MidiSource<'a>,
    {
        let msg = match status.byte() >> 4 {
            0x8 => VoiceEvent::NoteOff {
                key: Key::from_databyte(reader.read_next()?)?,
                velocity: Velocity::new(reader.read_next()?)?,
            },
            0x9 => {
                let key = reader.read_next()?;
                let velocity = reader.read_next()?;

                VoiceEvent::NoteOn {
                    key: Key::from_databyte(key)?,
                    velocity: Velocity::new(velocity)?,
                }
            }
            0xA => VoiceEvent::Aftertouch {
                key: Key::from_databyte(reader.read_next()?)?,
                velocity: Velocity::new(reader.read_next()?)?,
            },
            0xB => VoiceEvent::ControlChange {
                controller: Controller::new(reader.read_next()?)?,
                value: reader.read_next()?.try_into()?,
            },
            0xC => VoiceEvent::ProgramChange {
                program: Program::new(reader.read_next()?)?,
            },
            0xD => VoiceEvent::ChannelPressureAfterTouch {
                velocity: Velocity::new(reader.read_next()?)?,
            },
            0xE => {
                //Note the little-endian order, contrasting with the default big-endian order of
                //Standard Midi Files
                let b = reader.read_exact(2)?;
                let lsb = b[0];
                let msb = b[1];
                VoiceEvent::PitchBend(PitchBend::new_unchecked(lsb, msb))
            }
            b => {
                return Err(inv_data(
                    reader,
                    format!("Invalid status byte for message: {}", b),
                ));
            }
        };
        Ok(ChannelVoiceMessage {
            status,
            message: msg,
        })
    }

    /// Get the channel for the event
    pub fn channel(&self) -> Channel {
        Channel::from_status(self.status.byte())
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
    pub fn key(&self) -> Option<&Key> {
        match &self.message {
            VoiceEvent::NoteOn { key, .. }
            | VoiceEvent::NoteOff { key, .. }
            | VoiceEvent::Aftertouch { key, .. } => Some(key),
            _ => None,
        }
    }

    /// Returns the velocity if the type has a velocity
    pub fn velocity(&self) -> Option<&Velocity> {
        match &self.message {
            VoiceEvent::NoteOn { velocity, .. }
            | VoiceEvent::NoteOff { velocity, .. }
            | VoiceEvent::Aftertouch { velocity, .. }
            | VoiceEvent::ChannelPressureAfterTouch { velocity } => Some(velocity),
            _ => None,
        }
    }

    /// Returns the byte value for the data 1 byte. In the case
    /// of voice message it always exists
    pub fn data_1_byte(&self) -> DataByte {
        use VoiceEvent as V;
        match &self.message {
            V::NoteOn { key, .. } | V::NoteOff { key, .. } | V::Aftertouch { key, .. } => {
                key.byte()
            }
            V::ControlChange { controller, .. } => controller.byte(),
            V::ProgramChange { program } => program.byte(),
            V::ChannelPressureAfterTouch { velocity } => velocity.byte(),
            V::PitchBend(p) => p.lsb(),
        }
    }

    /// Returns the byte value for the data 2 byte if it exists
    pub fn data_2_byte(&self) -> Option<DataByte> {
        match &self.message {
            VoiceEvent::NoteOn { velocity, .. }
            | VoiceEvent::NoteOff { velocity, .. }
            | VoiceEvent::Aftertouch { velocity, .. }
            | VoiceEvent::ChannelPressureAfterTouch { velocity } => Some(velocity.byte()),
            VoiceEvent::ControlChange { value, .. } => Some(*value),
            VoiceEvent::PitchBend(p) => Some(p.msb()),
            _ => None,
        }
    }

    /// References the status byte of the event in big-endian.
    ///
    /// the leading (msb) 4 bytes are the voice event
    /// and the trailing (lsb) 4 bytes are the channel
    pub fn status(&self) -> u8 {
        self.status.byte()
    }

    /// References the voice event for the message.
    pub fn event(&self) -> &VoiceEvent {
        &self.message
    }

    /// Get the raw midi packet for this message
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut packet = Vec::with_capacity(3);
        packet.push(self.status());
        let data = self.message.to_raw().into_iter().map(|b| b.value());
        packet.extend(data);

        packet
    }
}

impl FromLiveEventBytes for ChannelVoiceMessage {
    const MIN_STATUS_BYTE: u8 = 0x80;
    const MAX_STATUS_BYTE: u8 = 0xEF;
    fn from_status_and_data(status: u8, data: &[u8]) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        let msg = match status >> 4 {
            0x8 => VoiceEvent::NoteOff {
                key: Key::from_databyte(
                    data.get_byte(0)
                        .ok_or(io::Error::new(ErrorKind::InvalidData, "byte not found"))?,
                )?,
                velocity: Velocity::new(
                    data.get_byte(1)
                        .ok_or(io::Error::new(ErrorKind::InvalidData, "byte not found"))?,
                )?,
            },
            0x9 => VoiceEvent::NoteOn {
                key: Key::from_databyte(
                    data.get_byte(0)
                        .ok_or(io::Error::new(ErrorKind::InvalidData, "byte not found"))?,
                )?,
                velocity: Velocity::new(
                    data.get_byte(1)
                        .ok_or(io::Error::new(ErrorKind::InvalidData, "byte not found"))?,
                )?,
            },
            0xA => VoiceEvent::Aftertouch {
                key: Key::from_databyte(
                    data.get_byte(0)
                        .ok_or(io::Error::new(ErrorKind::InvalidData, "byte not found"))?,
                )?,
                velocity: Velocity::new(
                    data.get_byte(1)
                        .ok_or(io::Error::new(ErrorKind::InvalidData, "byte not found"))?,
                )?,
            },
            0xB => VoiceEvent::ControlChange {
                controller: Controller::new(
                    data.get_byte(0)
                        .ok_or(io::Error::new(ErrorKind::InvalidData, "byte not found"))?,
                )?,
                value: (data
                    .get_byte(1)
                    .ok_or(io::Error::new(ErrorKind::InvalidData, "byte not found"))?)
                .try_into()?,
            },
            0xC => VoiceEvent::ProgramChange {
                program: Program::new(
                    data.get_byte(0)
                        .ok_or(io::Error::new(ErrorKind::InvalidData, "byte not found"))?,
                )?,
            },
            0xD => VoiceEvent::ChannelPressureAfterTouch {
                velocity: Velocity::new(
                    data.get_byte(0)
                        .ok_or(io::Error::new(ErrorKind::InvalidData, "byte not found"))?,
                )?,
            },
            0xE => {
                //Note the little-endian order, contrasting with the default big-endian order of
                //Standard Midi Files
                let lsb = data
                    .get_byte(0)
                    .ok_or(io::Error::new(ErrorKind::InvalidData, "byte not found"))?;
                let msb = data
                    .get_byte(1)
                    .ok_or(io::Error::new(ErrorKind::InvalidData, "byte not found"))?;
                VoiceEvent::PitchBend(PitchBend::new(lsb, msb)?)
            }
            _ => panic!("parsed midi message before checking that status is in range"),
        };
        Ok(ChannelVoiceMessage {
            status: status.try_into()?,
            message: msg,
        })
    }
}
