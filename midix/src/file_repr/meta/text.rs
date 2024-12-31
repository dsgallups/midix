use core::fmt;
use std::{
    borrow::Cow,
    io::{self, ErrorKind},
};

/// Some text, usually identified by a ['MetaMessage'](super::MetaMessage)s
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BytesText<'a> {
    inner: Cow<'a, str>,
}

impl<'a> BytesText<'a> {
    /// Interpret a byte slice as some text.
    pub fn new_from_byte_slice(bytes: &'a [u8]) -> Result<Self, io::Error> {
        let text = std::str::from_utf8(bytes).map_err(|e| {
            io::Error::new(ErrorKind::InvalidData, format!("Invalid string: {:?}", e))
        })?;
        Ok(Self {
            inner: Cow::Borrowed(text),
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
