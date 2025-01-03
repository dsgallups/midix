use crate::prelude::*;

#[doc = r#"
The channel mode message, capable of reserved functions

# Overview
This looks a lot like [`VoiceEvent::ControlChange`].

Both have identical status bytes (`1011xxxx`),

However, some values in the data bytes are reserved
for specific purposes. Those values can be handled
using this message.

# Reserved Controller numbers
Local Control:

When Local Control is Off, all devices on a given channel
will respond only to data received over MIDI. Played
data, etc. will be ignored. Local Control On restores
the functions of the normal controllers.

c = 122, v = 0: Local Control Off

c = 122, v = 127: Local Control On
---

All Notes Off:

When an All Notes Off is received, all oscillators
will turn off.

c = 123, v = 0: All Notes Off

c = 124, v = 0: Omni Mode Off

c = 125, v = 0: Omni Mode On

c = 126, v = M: Mono Mode On (Poly Off) where M is
the number of channels (Omni Off) or 0 (Omni On)

c = 127, v = 0: Poly Mode On (Mono Off) (Note:
These four messages also cause All Notes Off)
"#]
#[allow(dead_code)]
#[derive(Debug)]
pub struct ChannelModeMessage {
    controller: Controller,
    value: DataByte,
}
