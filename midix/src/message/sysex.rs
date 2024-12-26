use crate::bytes::AsMidiBytes;

pub trait SystemExclusive {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct SystemExclusiveOwned(Vec<u8>);

impl SystemExclusiveOwned {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

impl SystemExclusive for SystemExclusiveOwned {
    fn len(&self) -> usize {
        self.0.len()
    }
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl AsMidiBytes for SystemExclusiveOwned {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.len() + 2);
        bytes.push(0xF0);
        bytes.extend(&self.0);
        bytes.push(0xF7);
        bytes
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct SystemExclusiveBorrowed<'a>(&'a [u8]);

impl SystemExclusive for SystemExclusiveBorrowed<'_> {
    fn len(&self) -> usize {
        self.0.len()
    }
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl AsMidiBytes for SystemExclusiveBorrowed<'_> {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.len() + 2);
        bytes.push(0xF0);
        bytes.extend(self.0);
        bytes.push(0xF7);
        bytes
    }
}
