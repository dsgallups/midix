use core::fmt::{self, Debug};

use alloc::{borrow::Cow, string::String};

use crate::ParseError;

/// Some text, usually identified by a ['MetaMessage'](super::MetaMessage)s
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BytesText<'a> {
    inner: Cow<'a, [u8]>,
}

impl<'a> BytesText<'a> {
    /// Interpret a byte slice as some text.
    pub fn new_from_bytes<B: Into<Cow<'a, [u8]>>>(bytes: B) -> Self {
        Self {
            inner: bytes.into(),
        }
    }

    /// Get a mutable reference to the underlying string
    pub fn to_mut(&mut self) -> Result<&mut str, ParseError> {
        let inner_mut = self.inner.to_mut();
        core::str::from_utf8_mut(inner_mut).map_err(|_| ParseError::InvalidUtf8)
    }

    /// Get a string reference
    pub const fn as_str(&self) -> Result<&str, ParseError> {
        let Ok(res) = (match &self.inner {
            Cow::Borrowed(inner) => str::from_utf8(inner),
            Cow::Owned(inner) => str::from_utf8(inner.as_slice()),
        }) else {
            return Err(ParseError::InvalidUtf8);
        };
        Ok(res)
    }

    /// Get a (possibly) cloned string
    pub fn into_string(self) -> Result<String, ParseError> {
        String::from_utf8(self.inner.into_owned()).map_err(|_| ParseError::InvalidUtf8)
    }
}

impl fmt::Display for BytesText<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
