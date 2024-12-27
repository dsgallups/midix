use crate::{
    channel::Channel,
    parser::reader::{check_u7, inv_data, ReadResult, Reader},
    prelude::{Key, Velocity},
};

use super::voice_event::VoiceEvent;

/// Represents a MIDI voice message,.
///
/// If you wish to parse a MIDI message from a slice of raw MIDI bytes, use the
/// [`LiveEvent::parse`](live/enum.LiveEvent.html#method.parse) method instead and ignore all
/// variants except for [`LiveEvent::Midi`](live/enum.LiveEvent.html#variant.Midi).
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct ChannelVoice<'a> {
    /// The MIDI channel that this event is associated with.
    /// Used for getting the channel
    status: &'a u8,
    /// The MIDI message type and associated data.
    message: VoiceEvent<'a>,
}

impl<'a> ChannelVoice<'a> {
    /// TODO: read functions should take in an iterator that yields u8s
    pub fn read(reader: &mut Reader<&'a [u8]>) -> ReadResult<Self> {
        //bugged
        let status = reader.read_next()?;

        let msg = match status >> 4 {
            0x8 => VoiceEvent::NoteOff {
                key: check_u7(reader)?,
                vel: check_u7(reader)?,
            },
            0x9 => VoiceEvent::NoteOn {
                key: check_u7(reader)?,
                vel: check_u7(reader)?,
            },
            0xA => VoiceEvent::Aftertouch {
                key: check_u7(reader)?,
                vel: check_u7(reader)?,
            },
            0xB => VoiceEvent::ControlChange {
                controller: check_u7(reader)?,
                value: check_u7(reader)?,
            },
            0xC => VoiceEvent::ProgramChange {
                program: check_u7(reader)?,
            },
            0xD => VoiceEvent::ChannelPressureAfterTouch {
                vel: check_u7(reader)?,
            },
            0xE => {
                //Note the little-endian order, contrasting with the default big-endian order of
                //Standard Midi Files
                let [lsb, msb] = reader.read_exact_size()?;
                VoiceEvent::PitchBend { lsb, msb }
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
    pub fn key(&self) -> Option<Key> {
        match self.message {
            VoiceEvent::NoteOn { key, .. }
            | VoiceEvent::NoteOff { key, .. }
            | VoiceEvent::Aftertouch { key, .. } => Some(Key::new(*key)),
            _ => None,
        }
    }
    pub fn velocity(&self) -> Option<Velocity> {
        match self.message {
            VoiceEvent::NoteOn { vel, .. }
            | VoiceEvent::NoteOff { vel, .. }
            | VoiceEvent::Aftertouch { vel, .. }
            | VoiceEvent::ChannelPressureAfterTouch { vel } => Some(Velocity::new(*vel)),
            _ => None,
        }
    }

    pub fn status(&self) -> &u8 {
        //self.message.status_nibble() << 4 | self.channel.bits()
        self.status
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
