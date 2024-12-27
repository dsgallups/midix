mod sysex;
pub use sysex::*;

use crate::{message::ChannelVoiceRef, prelude::MetaRef};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TrackMessage<'a> {
    ChannelVoice(ChannelVoiceRef<'a>),
    SystemExclusive(SysEx<'a>),
    /// A meta-message, giving extra information for correct playback, like tempo, song name,
    /// lyrics, etc...
    Meta(MetaRef<'a>),
}
/*
impl<'a> TrackMessage<'a> {
    /*pub fn to_owned(self) -> MidiTrackMessage {
        use TrackMessage::*;
        match self {
            ChannelVoice(c) => MidiTrackMessage::ChannelVoice(c),
            SystemExclusive(s) => MidiTrackMessage::SystemExclusive(s.to_owned()),
            Meta(m) => MidiTrackMessage::Meta(m.to_owned()),
        }
    }*/
}*/
