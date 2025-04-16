use core::fmt;
use std::{
    borrow::Cow,
    io::{self},
};

/// Some text, usually identified by a ['MetaMessage'](super::MetaMessage)s
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BytesText<'a> {
    inner: Cow<'a, [u8]>,
}

impl<'a> BytesText<'a> {
    /// Interpret a byte slice as some text.
    pub fn new_from_bytes<B: Into<Cow<'a, [u8]>>>(bytes: B) -> Result<Self, io::Error> {
        Ok(Self {
            inner: bytes.into(),
        })
    }

    /// Get a mutable reference to the underlying string
    pub fn to_mut(&mut self) -> &mut String {
        self.inner.to_mut()
    }

    /// Get a string reference
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    /// Get a (possibly) cloned string
    pub fn into_string(self) -> String {
        self.inner.into_owned()
    }
}

impl fmt::Display for BytesText<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
