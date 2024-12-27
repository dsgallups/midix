mod sysex;
pub use sysex::*;
mod meta;
pub use meta::*;
mod voice;
pub use voice::*;
mod voice_event;
pub use voice_event::*;

use super::{ReadResult, Reader};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TrackMessage<'a> {
    ChannelVoice(ChannelVoiceMessage<'a>),
    SystemExclusive(SysEx<'a>),
    /// A meta-message, giving extra information for correct playback, like tempo, song name,
    /// lyrics, etc...
    Meta(Meta<'a>),
}

impl<'a> TrackMessage<'a> {
    pub fn read<'r, 'slc>(reader: &'r mut Reader<&'slc [u8]>) -> ReadResult<Self>
    where
        'slc: 'a,
    {
        //BUGGED, needs refactor
        let next_event = reader.peak_next()?;

        let res = match next_event {
            0xF0 => {
                //since we've peaked
                reader.increment_buffer_position(1);
                let mut data = reader.read_varlen_slice()?;
                if !data.is_empty() {
                    //discard the last 0xF7
                    data = &data[..data.len() - 1];
                }
                Self::SystemExclusive(SysEx::new(data))
            }
            0xFF => {
                //since we've peaked
                reader.increment_buffer_position(1);
                Self::Meta(Meta::read(reader)?)
            }
            _ => Self::ChannelVoice(ChannelVoiceMessage::read(reader)?),
        };

        Ok(res)
    }

    /*pub fn to_owned(self) -> MidiTrackMessage {
        use TrackMessage::*;
        match self {
            ChannelVoice(c) => MidiTrackMessage::ChannelVoice(c),
            SystemExclusive(s) => MidiTrackMessage::SystemExclusive(s.to_owned()),
            Meta(m) => MidiTrackMessage::Meta(m.to_owned()),
        }
    }*/
}
