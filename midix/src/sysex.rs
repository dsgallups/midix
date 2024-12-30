use std::borrow::Cow;

#[doc = r#"
A System Exclusive messsage, found in
both [`LiveEvent`](crate::prelude::LiveEvent)s and [`FileEvent`](crate::prelude::FileEvent)s.

# Overview
System Exclusive messages include a
Manufacturer's Identification (ID) code,
and are used to transfer any number of
data bytes in a format specified by the
referenced manufacturer.
"#]
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct SysExMessage<'a>(Cow<'a, [u8]>);

impl<'a> SysExMessage<'a> {
    /// Create a new owned system exclusive message
    pub const fn new(data: Vec<u8>) -> Self {
        Self(Cow::Owned(data))
    }

    /// Create a new system exclusive message from a borrowed slice
    pub const fn new_borrowed(data: &'a [u8]) -> Self {
        Self(Cow::Borrowed(data))
    }

    /// Returns a mutable reference to the underlying data.
    pub fn data_mut(&mut self) -> &mut Vec<u8> {
        self.0.to_mut()
    }

    /// Get the length of the sysex data
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// returns true without sysex data
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Interprets the sysex as a live-streamed set of bytes.
    ///
    /// Note that live bytes don't have an identifying length, unlike a file system common message.
    pub fn to_live_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.len() + 2);
        bytes.push(0xF0);
        bytes.extend(self.0.iter());
        bytes.push(0xF7);
        bytes
    }
}
