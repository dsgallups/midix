use std::borrow::Cow;

use crate::prelude::*;

/// Represents a MIDI voice message,.
///
/// If you wish to parse a MIDI message from a slice of raw MIDI bytes, use the
/// [`LiveEvent::parse`](live/enum.LiveEvent.html#method.parse) method instead and ignore all
/// variants except for [`LiveEvent::Midi`](live/enum.LiveEvent.html#variant.Midi).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ChannelVoiceMessage<'a> {
    /// The MIDI channel that this event is associated with.
    /// Used for getting the channel as the status' lsb contains the channel
    status: StatusByte<'a>,
    /// The MIDI message type and associated data.
    message: VoiceEvent<'a>,
}

impl<'a> ChannelVoiceMessage<'a> {
    /// Create a new channel voice event from the channel and associated event type
    pub fn new(channel: Channel<'_>, message: VoiceEvent<'a>) -> Self {
        let status = *channel.byte() | (message.status_nibble() << 4);
        Self {
            status: StatusByte::new_unchecked(status),
            message,
        }
    }

    /// TODO: read functions should take in an iterator that yields u8s
    pub(crate) fn read(status: Cow<'a, u8>, reader: &mut Reader<&'a [u8]>) -> ReadResult<Self> {
        let msg = match status.as_ref() >> 4 {
            0x8 => VoiceEvent::NoteOff {
                key: Key::new(reader.read_next()?)?,
                velocity: Velocity::new(reader.read_next()?)?,
            },
            0x9 => VoiceEvent::NoteOn {
                key: Key::new(reader.read_next()?)?,
                velocity: Velocity::new(reader.read_next()?)?,
            },
            0xA => VoiceEvent::Aftertouch {
                key: Key::new(reader.read_next()?)?,
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
                let [lsb, msb] = reader.read_exact_size()?;
                VoiceEvent::PitchBend(PitchBend::new_borrowed_unchecked(lsb, msb))
            }
            b => {
                return Err(inv_data(
                    reader,
                    format!("Invalid status byte for message: {}", b),
                ))
            }
        };
        Ok(ChannelVoiceMessage {
            status: status.try_into()?,
            message: msg,
        })
    }

    /// Get the channel for the event
    pub fn channel(&self) -> Channel {
        Channel::from_status(*self.status.byte())
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
    pub fn key(&self) -> Option<&Key<'a>> {
        match &self.message {
            VoiceEvent::NoteOn { key, .. }
            | VoiceEvent::NoteOff { key, .. }
            | VoiceEvent::Aftertouch { key, .. } => Some(key),
            _ => None,
        }
    }

    /// Returns the velocity if the type has a velocity
    pub fn velocity(&self) -> Option<&Velocity<'a>> {
        match &self.message {
            VoiceEvent::NoteOn { velocity, .. }
            | VoiceEvent::NoteOff { velocity, .. }
            | VoiceEvent::Aftertouch { velocity, .. }
            | VoiceEvent::ChannelPressureAfterTouch { velocity } => Some(velocity),
            _ => None,
        }
    }

    /// References the status byte of the event in big-endian.
    ///
    /// the leading (msb) 4 bytes are the voice event
    /// and the trailing (lsb) 4 bytes are the channel
    pub fn status(&self) -> &u8 {
        //self.message.status_nibble() << 4 | self.channel.bits()
        self.status.byte()
    }

    /// References the voice event for the message.
    pub fn event(&self) -> &VoiceEvent {
        &self.message
    }

    /// Get the raw midi packet for this message
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut packet = Vec::with_capacity(3);
        packet.push(*self.status());
        let data = self.message.to_raw();
        packet.extend(data);

        packet
    }
}

impl FromLiveEventBytes for ChannelVoiceMessage<'_> {
    const MIN_STATUS_BYTE: u8 = 0x80;
    const MAX_STATUS_BYTE: u8 = 0xEF;
    fn from_status_and_data(status: u8, data: &[u8]) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        let msg = match status >> 4 {
            0x8 => VoiceEvent::NoteOff {
                key: Key::new(*data.get_byte(0)?)?,
                velocity: Velocity::new(*data.get_byte(1)?)?,
            },
            0x9 => VoiceEvent::NoteOn {
                key: Key::new(*data.get_byte(0)?)?,
                velocity: Velocity::new(*data.get_byte(1)?)?,
            },
            0xA => VoiceEvent::Aftertouch {
                key: Key::new(*data.get_byte(0)?)?,
                velocity: Velocity::new(*data.get_byte(1)?)?,
            },
            0xB => VoiceEvent::ControlChange {
                controller: Controller::new(*data.get_byte(0)?)?,
                value: (*data.get_byte(1)?).try_into()?,
            },
            0xC => VoiceEvent::ProgramChange {
                program: Program::new(*data.get_byte(0)?)?,
            },
            0xD => VoiceEvent::ChannelPressureAfterTouch {
                velocity: Velocity::new(*data.get_byte(0)?)?,
            },
            0xE => {
                //Note the little-endian order, contrasting with the default big-endian order of
                //Standard Midi Files
                let lsb = *data.get_byte(0)?;
                let msb = *data.get_byte(1)?;
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
