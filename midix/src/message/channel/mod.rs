#![doc = r#"
Contains all Channel Message types

# Hierarchy
```text
                |-----------------|
                | Channel Message |
                |-----------------|
                 /               \
|-----------------------|   |----------------------|
| Channel Voice Message |   | Channel Mode Message |
|-----------------------|   |----------------------|
```
"#]
mod mode;
pub use mode::*;

mod voice;
pub use voice::*;
mod voice_event;
pub use voice_event::*;

#[doc = r#"
The set of possible Channel messages
"#]
pub enum ChannelMessage {
    /// A channel voice message
    Voice(ChannelVoiceMessage),
    /// A channel mode message
    Mode(ChannelModeMessage),
}

impl From<ChannelVoiceMessage> for ChannelMessage {
    fn from(value: ChannelVoiceMessage) -> Self {
        Self::Voice(value)
    }
}

impl From<ChannelModeMessage> for ChannelMessage {
    fn from(value: ChannelModeMessage) -> Self {
        Self::Mode(value)
    }
}
