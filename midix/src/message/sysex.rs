use crate::bytes::AsMidiBytes;

pub trait SystemExclusiveTrait {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct SysEx(Vec<u8>);

impl SysEx {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

impl SystemExclusiveTrait for SysEx {
    fn len(&self) -> usize {
        self.0.len()
    }
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl AsMidiBytes for SysEx {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.len() + 2);
        bytes.push(0xF0);
        bytes.extend(&self.0);
        bytes.push(0xF7);
        bytes
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct SysExRef<'a>(&'a [u8]);

impl<'a> SysExRef<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self(data)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.len() + 2);
        bytes.push(0xF0);
        bytes.extend(self.0);
        bytes.push(0xF7);
        bytes
    }
}
