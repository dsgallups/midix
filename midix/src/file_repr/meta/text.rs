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
        let self_mut = self.inner.to_mut();
        #[cfg(feature = "std")]
        let _v = std::str::from_utf8_mut(self_mut).map_err(|_| ParseError::InvalidUtf8)?;
        #[cfg(feature = "nightly")]
        let _v = str::from_utf8_mut(self_mut).map_err(|_| ParseError::InvalidUtf8)?;
        #[cfg(all(not(feature = "std"), not(feature = "nightly")))]
        panic!("cannot get mutable reference to text without `std` or `nightly` features enabled!",);
        Ok(_v)
    }

    /// Get a string reference
    pub fn as_str(&self) -> Result<&str, ParseError> {
        #[cfg(feature = "std")]
        let _v = std::str::from_utf8(&self.inner).map_err(|_| ParseError::InvalidUtf8)?;
        #[cfg(feature = "nightly")]
        let _v = str::from_utf8(&self.inner).map_err(|_| ParseError::InvalidUtf8)?;
        #[cfg(all(not(feature = "std"), not(feature = "nightly")))]
        panic!("Cannot intrepret string without the std or nightly feature");
        #[cfg(any(feature = "std", feature = "nightly"))]
        Ok(_v)
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
