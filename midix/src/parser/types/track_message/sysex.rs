#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct SysEx<'a>(&'a [u8]);

impl<'a> SysEx<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self(data)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.len() + 2);
        bytes.push(0xF0);
        bytes.extend(self.0);
        bytes.push(0xF7);
        bytes
    }
}
