use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MidiTrackMessageRef<'a> {
    ChannelVoice(ChannelVoiceMessage),
    SystemExclusive(SystemExclusiveRef<'a>),
    /// A meta-message, giving extra information for correct playback, like tempo, song name,
    /// lyrics, etc...
    Meta(MetaMessageRef<'a>),
}

impl<'a> MidiTrackMessageRef<'a> {
    pub fn read<'r, 'slc>(reader: &'r mut Reader<&'slc [u8]>) -> ReadResult<Self>
    where
        'slc: 'a,
    {
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
                Self::SystemExclusive(SystemExclusiveRef::new(data))
            }
            0xFF => {
                //since we've peaked
                reader.increment_buffer_position(1);
                Self::Meta(MetaMessageRef::read(reader)?)
            }
            _ => Self::ChannelVoice(ChannelVoiceMessage::read(reader)?),
        };

        Ok(res)
    }
}
