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
    message: VoiceEvent,
}

impl ChannelVoiceMessage {
    /*/// TODO: read functions should take in an iterator that yields u8s
        pub fn read(reader: &mut OldReader<&[u8]>) -> OldReadResult<Self> {
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
    */
    pub fn new(channel: Channel, message: VoiceEvent) -> Self {
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
            VoiceEvent::NoteOn { key, .. }
            | VoiceEvent::NoteOff { key, .. }
            | VoiceEvent::Aftertouch { key, .. } => Some(key),
            _ => None,
        }
    }
    pub fn velocity(&self) -> Option<Velocity> {
        match self.message {
            VoiceEvent::NoteOn { vel, .. }
            | VoiceEvent::NoteOff { vel, .. }
            | VoiceEvent::Aftertouch { vel, .. }
            | VoiceEvent::ChannelPressureAfterTouch { vel } => Some(vel),
            _ => None,
        }
    }

    pub fn status(&self) -> u8 {
        self.message.status_nibble() << 4 | self.channel.bits()
    }
    pub fn message(&self) -> &VoiceEvent {
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
            0x8 => VoiceEvent::NoteOff {
                key: Key::from_bits(*data.get_byte(0)?)?,
                vel: Velocity::from_bits(*data.get_byte(1)?)?,
            },
            0x9 => VoiceEvent::NoteOn {
                key: Key::from_bits(*data.get_byte(0)?)?,
                vel: Velocity::from_bits(*data.get_byte(1)?)?,
            },
            0xA => VoiceEvent::Aftertouch {
                key: Key::from_bits(*data.get_byte(0)?)?,
                vel: Velocity::from_bits(*data.get_byte(1)?)?,
            },
            0xB => VoiceEvent::ControlChange {
                controller: Controller::from_bits(*data.get_byte(0)?)?,
                value: check_u7(*data.get_byte(1)?)?,
            },
            0xC => VoiceEvent::ProgramChange {
                program: Program::from_bits(*data.get_byte(0)?)?,
            },
            0xD => VoiceEvent::ChannelPressureAfterTouch {
                vel: Velocity::from_bits(*data.get_byte(0)?)?,
            },
            0xE => {
                //Note the little-endian order, contrasting with the default big-endian order of
                //Standard Midi Files
                let lsb = *data.get_byte(0)?;
                let msb = *data.get_byte(1)?;
                VoiceEvent::PitchBend(PitchBend::from_byte_pair(lsb, msb)?)
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

/// Represents a MIDI voice message,.
///
/// If you wish to parse a MIDI message from a slice of raw MIDI bytes, use the
/// [`LiveEvent::parse`](live/enum.LiveEvent.html#method.parse) method instead and ignore all
/// variants except for [`LiveEvent::Midi`](live/enum.LiveEvent.html#variant.Midi).
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct ChannelVoice<'a> {
    /// The MIDI channel that this event is associated with.
    /// Used for getting the channel
    status: u8,
    /// The MIDI message type and associated data.
    message: VoiceEventRef<'a>,
}

impl<'a> ChannelVoice<'a> {
    /// TODO: read functions should take in an iterator that yields u8s
    pub(crate) fn read(status: u8, reader: &mut Reader<&'a [u8]>) -> ReadResult<Self> {
        use crate::parser::reader::check_u7;
        //bugged

        let msg = match status >> 4 {
            0x8 => VoiceEventRef::NoteOff {
                key: KeyRef::new(check_u7(reader)?),
                velocity: VelocityRef::new(check_u7(reader)?),
            },
            0x9 => VoiceEventRef::NoteOn {
                key: KeyRef::new(check_u7(reader)?),
                velocity: VelocityRef::new(check_u7(reader)?),
            },
            0xA => VoiceEventRef::Aftertouch {
                key: KeyRef::new(check_u7(reader)?),
                velocity: VelocityRef::new(check_u7(reader)?),
            },
            0xB => VoiceEventRef::ControlChange {
                controller: ControllerRef::new(check_u7(reader)?),
                value: check_u7(reader)?,
            },
            0xC => VoiceEventRef::ProgramChange {
                program: ProgramRef::new(check_u7(reader)?),
            },
            0xD => VoiceEventRef::ChannelPressureAfterTouch {
                velocity: VelocityRef::new(check_u7(reader)?),
            },
            0xE => {
                //Note the little-endian order, contrasting with the default big-endian order of
                //Standard Midi Files
                let [lsb, msb] = reader.read_exact_size()?;
                VoiceEventRef::PitchBend(PitchBendRef::new(lsb, msb))
            }
            b => {
                return Err(inv_data(
                    reader,
                    format!("Invalid status byte for message: {}", b),
                ))
            }
        };
        Ok(ChannelVoice {
            status,
            message: msg,
        })
    }
    pub fn channel(&self) -> Channel {
        Channel::from_status(self.status)
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
    pub fn key(&self) -> Option<KeyRef<'a>> {
        match self.message {
            VoiceEventRef::NoteOn { key, .. }
            | VoiceEventRef::NoteOff { key, .. }
            | VoiceEventRef::Aftertouch { key, .. } => Some(key),
            _ => None,
        }
    }
    pub fn velocity(&self) -> Option<VelocityRef<'a>> {
        match self.message {
            VoiceEventRef::NoteOn { velocity, .. }
            | VoiceEventRef::NoteOff { velocity, .. }
            | VoiceEventRef::Aftertouch { velocity, .. }
            | VoiceEventRef::ChannelPressureAfterTouch { velocity } => Some(velocity),
            _ => None,
        }
    }

    pub fn status(&self) -> &u8 {
        //self.message.status_nibble() << 4 | self.channel.bits()
        &self.status
    }
    pub fn message(&self) -> &VoiceEventRef {
        &self.message
    }

    /*/// Get the raw midi packet for this message
    fn as_bytes(&self) -> Vec<u8> {
        let mut packet = Vec::with_capacity(3);
        packet.push(self.status());
        let data = self.message.to_raw();
        packet.extend(data);

        packet
    }*/
}
