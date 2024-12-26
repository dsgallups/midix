use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TrackMidiMessage {
    ChannelVoice(ChannelVoiceMessage),
    SystemExclusive(Vec<u8>),
    SystemRealTime(SystemRealTimeMessage),
}
