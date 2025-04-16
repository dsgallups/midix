use crate::{
    prelude::*,
    reader::{ReadError, ReaderError, ReaderErrorKind},
};

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
    event: VoiceEvent,
}

impl ChannelVoiceMessage {
    /// Create a new channel voice event from the channel and associated event type
    pub fn new(channel: Channel, message: VoiceEvent) -> Self {
        let status = channel.to_byte() | (message.status_nibble() << 4);
        Self {
            status: StatusByte::new_unchecked(status),
            event: message,
        }
    }

    /// TODO: read functions should take in an iterator that yields u8s
    pub(crate) fn read<'a, R>(status: StatusByte, reader: &mut Reader<R>) -> ReadResult<Self>
    where
        R: MidiSource<'a>,
    {
        let msg = match status.byte() >> 4 {
            0x8 => VoiceEvent::NoteOff {
                key: Key::from_databyte(reader.read_next()?)
                    .map_err(|v| ReaderError::parse_error(reader.buffer_position(), v))?,
                velocity: Velocity::new(reader.read_next()?)
                    .map_err(|v| ReaderError::parse_error(reader.buffer_position(), v))?,
            },
            0x9 => {
                let key = reader.read_next()?;
                let velocity = reader.read_next()?;

                VoiceEvent::NoteOn {
                    key: Key::from_databyte(key)
                        .map_err(|v| ReaderError::parse_error(reader.buffer_position(), v))?,
                    velocity: Velocity::new(velocity)
                        .map_err(|v| ReaderError::parse_error(reader.buffer_position(), v))?,
                }
            }
            0xA => VoiceEvent::Aftertouch {
                key: Key::from_databyte(reader.read_next()?)
                    .map_err(|v| ReaderError::parse_error(reader.buffer_position(), v))?,
                velocity: Velocity::new(reader.read_next()?)
                    .map_err(|v| ReaderError::parse_error(reader.buffer_position(), v))?,
            },
            0xB => VoiceEvent::ControlChange(Controller::read(reader)?),
            0xC => VoiceEvent::ProgramChange {
                program: Program::new(reader.read_next()?)
                    .map_err(|v| ReaderError::parse_error(reader.buffer_position(), v))?,
            },
            0xD => VoiceEvent::ChannelPressureAfterTouch {
                velocity: Velocity::new(reader.read_next()?)
                    .map_err(|v| ReaderError::parse_error(reader.buffer_position(), v))?,
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
                return Err(inv_data(reader, ParseError::InvalidStatusByte(b)));
            }
        };
        Ok(ChannelVoiceMessage { status, event: msg })
    }

    /// Get the channel for the event
    pub fn channel(&self) -> Channel {
        Channel::from_status(self.status.byte())
    }

    /// Returns true if the note is on. This excludes note on where the velocity is zero.
    pub fn is_note_on(&self) -> bool {
        self.event.is_note_on()
    }

    /// Returns true if the note is off. This includes note on where the velocity is zero.
    pub fn is_note_off(&self) -> bool {
        self.event.is_note_off()
    }

    /// Returns the key if the event has a key
    pub fn key(&self) -> Option<&Key> {
        match &self.event {
            VoiceEvent::NoteOn { key, .. }
            | VoiceEvent::NoteOff { key, .. }
            | VoiceEvent::Aftertouch { key, .. } => Some(key),
            _ => None,
        }
    }

    /// Returns the velocity if the type has a velocity
    pub fn velocity(&self) -> Option<&Velocity> {
        match &self.event {
            VoiceEvent::NoteOn { velocity, .. }
            | VoiceEvent::NoteOff { velocity, .. }
            | VoiceEvent::Aftertouch { velocity, .. }
            | VoiceEvent::ChannelPressureAfterTouch { velocity } => Some(velocity),
            _ => None,
        }
    }

    /// Returns the byte value for the data 1 byte. In the case
    /// of voice message it always exists
    pub fn data_1_byte(&self) -> u8 {
        use VoiceEvent as V;
        match &self.event {
            V::NoteOn { key, .. } | V::NoteOff { key, .. } | V::Aftertouch { key, .. } => {
                key.byte()
            }
            V::ControlChange(c) => c.to_bytes()[0],
            V::ProgramChange { program } => program.byte(),
            V::ChannelPressureAfterTouch { velocity } => velocity.byte(),
            V::PitchBend(p) => p.lsb(),
        }
    }

    /// Returns the byte value for the data 2 byte if it exists
    pub fn data_2_byte(&self) -> Option<u8> {
        match &self.event {
            VoiceEvent::NoteOn { velocity, .. }
            | VoiceEvent::NoteOff { velocity, .. }
            | VoiceEvent::Aftertouch { velocity, .. }
            | VoiceEvent::ChannelPressureAfterTouch { velocity } => Some(velocity.byte()),
            VoiceEvent::ControlChange(c) => c.to_bytes().get(1).copied(),
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
        &self.event
    }

    // /// Get the raw midi packet for this message
    // pub fn to_bytes(&self) -> Vec<u8> {
    //     let mut packet = Vec::with_capacity(3);
    //     packet.push(self.status());
    //     packet.extend(self.event.to_raw());

    //     packet
    // }
}

impl FromLiveEventBytes for ChannelVoiceMessage {
    const MIN_STATUS_BYTE: u8 = 0x80;
    const MAX_STATUS_BYTE: u8 = 0xEF;
    fn from_status_and_data(status: u8, data: &[u8]) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let msg = match status >> 4 {
            0x8 => VoiceEvent::NoteOff {
                key: Key::from_databyte(data.get_byte(0).ok_or(ParseError::MissingData)?)?,
                velocity: Velocity::new(data.get_byte(1).ok_or(ParseError::MissingData)?)?,
            },
            0x9 => VoiceEvent::NoteOn {
                key: Key::from_databyte(data.get_byte(0).ok_or(ParseError::MissingData)?)?,
                velocity: Velocity::new(data.get_byte(1).ok_or(ParseError::MissingData)?)?,
            },
            0xA => VoiceEvent::Aftertouch {
                key: Key::from_databyte(data.get_byte(0).ok_or(ParseError::MissingData)?)?,
                velocity: Velocity::new(data.get_byte(1).ok_or(ParseError::MissingData)?)?,
            },
            0xB => {
                // TODO: really need to unify this
                let mut temp = Reader::from_byte_slice(data);
                let c = Controller::read(&mut temp).map_err(|e| match e.kind {
                    ReaderErrorKind::ParseError(p) => p,
                    ReaderErrorKind::ReadError(p) => match p {
                        ReadError::OutOfBounds => ParseError::MissingData,
                    },
                })?;
                VoiceEvent::ControlChange(c)
            }
            0xC => VoiceEvent::ProgramChange {
                program: Program::new(data.get_byte(0).ok_or(ParseError::MissingData)?)?,
            },
            0xD => VoiceEvent::ChannelPressureAfterTouch {
                velocity: Velocity::new(data.get_byte(0).ok_or(ParseError::MissingData)?)?,
            },
            0xE => {
                //Note the little-endian order, contrasting with the default big-endian order of
                //Standard Midi Files
                let lsb = data.get_byte(0).ok_or(ParseError::MissingData)?;
                let msb = data.get_byte(1).ok_or(ParseError::MissingData)?;
                VoiceEvent::PitchBend(PitchBend::new(lsb, msb)?)
            }
            _ => panic!("parsed midi message before checking that status is in range"),
        };
        Ok(ChannelVoiceMessage {
            status: status.try_into()?,
            event: msg,
        })
    }
}
