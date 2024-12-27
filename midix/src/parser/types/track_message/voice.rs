use crate::{
    channel::Channel,
    message::VoiceEventRef,
    parser::reader::{check_u7, inv_data, ReadResult, Reader},
    prelude::{ControllerRef, KeyRef, PitchBendRef, ProgramRef, VelocityRef},
};

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
    pub fn read(status: u8, reader: &mut Reader<&'a [u8]>) -> ReadResult<Self> {
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
