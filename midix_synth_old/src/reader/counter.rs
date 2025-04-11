use std::io::Read;

pub struct ReadCounter<'a, R: Read + ?Sized> {
    reader: &'a mut R,
    count: usize,
}

impl<'a, R: Read + ?Sized> ReadCounter<'a, R> {
    pub fn new(reader: &'a mut R) -> Self {
        Self { reader, count: 0 }
    }

    pub fn bytes_read(&self) -> usize {
        self.count
    }
}

impl<R: Read + ?Sized> Read for ReadCounter<'_, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.count += len;
        Ok(len)
    }
}
