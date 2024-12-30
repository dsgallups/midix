#![doc = r#"
Contains all Channel Message types

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
pub enum ChannelMessage<'a> {
    /// A channel voice message
    Voice(ChannelVoiceMessage<'a>),
    /// A channel mode message
    Mode(ChannelModeMessage<'a>),
}
