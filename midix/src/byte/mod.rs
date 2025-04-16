mod message;
use std::{
    borrow::Cow,
    io::{self, ErrorKind},
    ops::Deref,
};

pub use message::*;

#[doc = r#"
Wraps a `Cow<'_, u8>`.

This is because Cow doesn't implement `From<Vec<u8>>` or `From<&[u8]>`, and a common interface is nice to have
for [`MidiSource`](crate::reader::MidiSource).
"#]
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Bytes<'a>(Cow<'a, [u8]>);

impl<'a> Bytes<'a> {
    /// Create a new set of bytes
    pub fn new<T: Into<Bytes<'a>>>(v: T) -> Self {
        v.into()
    }

    /// Removes X elements from the end
    pub fn truncate(&mut self, amt: usize) {
        match &mut self.0 {
            Cow::Borrowed(val) => {
                *val = &val[..val.len() - amt];
            }
            Cow::Owned(val) => {
                val.truncate(amt);
            }
        }
    }

    /// Returns mutable reference to underlying byte slice
    pub fn to_mut(&mut self) -> &mut Vec<u8> {
        self.0.to_mut()
    }

    /// Return a reference to the underlying Cow
    pub fn as_cow(&self) -> &Cow<'a, [u8]> {
        &self.0
    }

    /// Returns the underlying Cow
    pub fn into_inner(self) -> Cow<'a, [u8]> {
        self.0
    }

    /// Returns the underlying byte vec. Copies if borrowed.
    pub fn into_owned(self) -> Vec<u8> {
        self.0.into_owned()
    }
}

impl<'a> Deref for Bytes<'a> {
    type Target = <Cow<'a, [u8]> as Deref>::Target;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<u8>> for Bytes<'_> {
    fn from(value: Vec<u8>) -> Self {
        Self(Cow::Owned(value))
    }
}

impl<'a> From<&'a [u8]> for Bytes<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl<'a, const SIZE: usize> From<&'a [u8; SIZE]> for Bytes<'a> {
    fn from(value: &'a [u8; SIZE]) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl<const SIZE: usize> From<[u8; SIZE]> for Bytes<'_> {
    fn from(value: [u8; SIZE]) -> Self {
        Self(Cow::Owned(value.to_vec()))
    }
}

impl<'a> TryFrom<Bytes<'a>> for Cow<'a, str> {
    type Error = io::Error;
    fn try_from(value: Bytes<'a>) -> Result<Self, Self::Error> {
        match value.0 {
            Cow::Borrowed(value) => {
                let text = std::str::from_utf8(value).map_err(|e| {
                    io::Error::new(ErrorKind::InvalidData, format!("Invalid string: {:?}", e))
                })?;
                Ok(Cow::Borrowed(text))
            }
            Cow::Owned(v) => {
                let text = String::from_utf8(v).map_err(|e| {
                    io::Error::new(ErrorKind::InvalidData, format!("Invalid string: {:?}", e))
                })?;
                Ok(Cow::Owned(text))
            }
        }
    }
}

#[doc = r#"
A representation of a statically borrowed or owned array
"#]
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct BytesConst<'a, const SIZE: usize>(Cow<'a, [u8; SIZE]>);

impl<'a, const SIZE: usize> BytesConst<'a, SIZE> {
    /// Create a new set of bytes
    pub fn new<T: Into<BytesConst<'a, SIZE>>>(v: T) -> Self {
        v.into()
    }

    /// Returns mutable reference to underlying byte slice
    pub fn to_mut(&mut self) -> &mut [u8; SIZE] {
        self.0.to_mut()
    }

    /// Returns the underlying Cow
    pub fn into_inner(self) -> Cow<'a, [u8; SIZE]> {
        self.0
    }

    /// Returns the underlying byte vec. Copies if borrowed.
    pub fn into_owned(self) -> [u8; SIZE] {
        self.0.into_owned()
    }
}
impl<'a, const SIZE: usize> Deref for BytesConst<'a, SIZE> {
    type Target = <Cow<'a, [u8; SIZE]> as Deref>::Target;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, const SIZE: usize> TryFrom<Bytes<'a>> for BytesConst<'a, SIZE> {
    type Error = ();
    fn try_from(value: Bytes<'a>) -> Result<Self, Self::Error> {
        Ok(match value.into_inner() {
            Cow::Borrowed(v) => Self(Cow::Borrowed(v.try_into().map_err(|_| ())?)),
            Cow::Owned(v) => Self(Cow::Owned(v.try_into().map_err(|_| ())?)),
        })
    }
}

impl<const SIZE: usize> From<[u8; SIZE]> for BytesConst<'_, SIZE> {
    fn from(value: [u8; SIZE]) -> Self {
        Self(Cow::Owned(value))
    }
}

impl<'a, const SIZE: usize> From<&'a [u8; SIZE]> for BytesConst<'a, SIZE> {
    fn from(value: &'a [u8; SIZE]) -> Self {
        Self(Cow::Borrowed(value))
    }
}
