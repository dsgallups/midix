use std::borrow::Cow;

use crate::prelude::*;
mod event;
pub use event::*;

/// Represents a MIDI voice message,.
///
/// If you wish to parse a MIDI message from a slice of raw MIDI bytes, use the
/// [`LiveEvent::parse`](live/enum.LiveEvent.html#method.parse) method instead and ignore all
/// variants except for [`LiveEvent::Midi`](live/enum.LiveEvent.html#variant.Midi).
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ChannelVoice<'a> {
    /// The MIDI channel that this event is associated with.
    /// Used for getting the channel as the status' lsb contains the channel
    status: Cow<'a, u8>,
    /// The MIDI message type and associated data.
    message: VoiceEvent<'a>,
}

impl<'a> ChannelVoice<'a> {
    pub fn new(channel: Channel<'_>, message: VoiceEvent<'a>) -> Self {
        let status = *channel.byte() | (message.status_nibble() << 4);
        Self {
            status: Cow::Owned(status),
            message,
        }
    }

    /// TODO: read functions should take in an iterator that yields u8s
    pub(crate) fn read(status: Cow<'a, u8>, reader: &mut Reader<&'a [u8]>) -> ReadResult<Self> {
        use crate::reader::check_u7;
        //bugged

        let msg = match status.as_ref() >> 4 {
            0x8 => VoiceEvent::NoteOff {
                key: Key::new_borrowed(check_u7(reader)?),
                velocity: Velocity::new_borrowed(check_u7(reader)?),
            },
            0x9 => VoiceEvent::NoteOn {
                key: Key::new_borrowed(check_u7(reader)?),
                velocity: Velocity::new_borrowed(check_u7(reader)?),
            },
            0xA => VoiceEvent::Aftertouch {
                key: Key::new_borrowed(check_u7(reader)?),
                velocity: Velocity::new_borrowed(check_u7(reader)?),
            },
            0xB => VoiceEvent::ControlChange {
                controller: Controller::new_borrowed(check_u7(reader)?),
                value: Cow::Borrowed(check_u7(reader)?),
            },
            0xC => VoiceEvent::ProgramChange {
                program: Program::new_borrowed(check_u7(reader)?),
            },
            0xD => VoiceEvent::ChannelPressureAfterTouch {
                velocity: Velocity::new_borrowed(check_u7(reader)?),
            },
            0xE => {
                //Note the little-endian order, contrasting with the default big-endian order of
                //Standard Midi Files
                let [lsb, msb] = reader.read_exact_size()?;
                VoiceEvent::PitchBend(PitchBend::new_borrowed(lsb, msb))
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
        Channel::from_status(*self.status)
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
    pub fn velocity(&self) -> Option<&Velocity<'a>> {
        match &self.message {
            VoiceEvent::NoteOn { velocity, .. }
            | VoiceEvent::NoteOff { velocity, .. }
            | VoiceEvent::Aftertouch { velocity, .. }
            | VoiceEvent::ChannelPressureAfterTouch { velocity } => Some(velocity),
            _ => None,
        }
    }

    pub fn status(&self) -> &u8 {
        //self.message.status_nibble() << 4 | self.channel.bits()
        &self.status
    }
    pub fn message(&self) -> &VoiceEvent {
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
impl AsMidiBytes for ChannelVoice<'_> {
    /// Get the raw midi packet for this message
    fn as_bytes(&self) -> Vec<u8> {
        let mut packet = Vec::with_capacity(3);
        packet.push(*self.status());
        let data = self.message.to_raw();
        packet.extend(data);

        packet
    }
}

impl FromMidiMessage for ChannelVoice<'_> {
    const MIN_STATUS_BYTE: u8 = 0x80;
    const MAX_STATUS_BYTE: u8 = 0xEF;
    fn from_status_and_data(status: u8, data: &[u8]) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        let msg = match status >> 4 {
            0x8 => VoiceEvent::NoteOff {
                key: Key::from_bits(*data.get_byte(0)?)?,
                velocity: Velocity::from_bits(*data.get_byte(1)?)?,
            },
            0x9 => VoiceEvent::NoteOn {
                key: Key::from_bits(*data.get_byte(0)?)?,
                velocity: Velocity::from_bits(*data.get_byte(1)?)?,
            },
            0xA => VoiceEvent::Aftertouch {
                key: Key::from_bits(*data.get_byte(0)?)?,
                velocity: Velocity::from_bits(*data.get_byte(1)?)?,
            },
            0xB => VoiceEvent::ControlChange {
                controller: Controller::from_bits(*data.get_byte(0)?)?,
                value: Cow::Owned(check_u7(*data.get_byte(1)?)?),
            },
            0xC => VoiceEvent::ProgramChange {
                program: Program::from_bits(*data.get_byte(0)?)?,
            },
            0xD => VoiceEvent::ChannelPressureAfterTouch {
                velocity: Velocity::from_bits(*data.get_byte(0)?)?,
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
        Ok(ChannelVoice {
            status: Cow::Owned(status),
            message: msg,
        })
    }
}
