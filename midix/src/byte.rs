use std::borrow::Cow;

#[doc = r#"
Identifies a byte that follows the MIDI spec:

Status Byte
(80H - FFH)

or

Data Byte
(00H - 7FH)
"#]
#[allow(dead_code)]
pub struct MidiByte<'a>(Cow<'a, u8>);
