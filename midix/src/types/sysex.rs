use std::borrow::Cow;

use crate::prelude::*;

#[doc = r#"
A System Exclusive messsage, found in
both [`LiveEvent`]s and [`FileEvent`]s.

# Overview
System Exclusive messages include a
Manufacturer's Identification (ID) code,
and are used to transfer any number of
data bytes in a format specified by the
referenced manufacturer.
"#]
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct SysEx<'a>(Cow<'a, [u8]>);

impl<'a> SysEx<'a> {
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
