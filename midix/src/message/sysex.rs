use std::borrow::Cow;

use crate::bytes::AsMidiBytes;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct SysEx<'a>(Cow<'a, [u8]>);

impl<'a> SysEx<'a> {
    pub const fn new(data: Vec<u8>) -> Self {
        Self(Cow::Owned(data))
    }
    pub const fn new_borrowed(data: &'a [u8]) -> Self {
        Self(Cow::Borrowed(data))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl AsMidiBytes for SysEx<'_> {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.len() + 2);
        bytes.push(0xF0);
        bytes.extend(self.0.iter());
        bytes.push(0xF7);
        bytes
    }
}
