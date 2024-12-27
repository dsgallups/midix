use std::ops::Neg;

#[doc = r#"
FF 59 02 sf mi Key Signature
sf = -7: 7 flats
sf = -1: 1 flat
sf = 0: key of C
sf = 1: 1 sharp
sf = 7: 7 sharps

mi = 0: major key
mi = 1: minor key
"#]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct KeySignatureRef<'a>(&'a [u8; 2]);
impl<'a> KeySignatureRef<'a> {
    pub fn new(v: &'a [u8; 2]) -> Self {
        Self(v)
    }
    pub fn sharp_flat_count(&self) -> i8 {
        self.0[0] as i8
    }

    pub fn num_sharps(&self) -> u8 {
        self.sharp_flat_count().min(0).unsigned_abs()
    }
    pub fn num_flats(&self) -> u8 {
        self.sharp_flat_count().neg().min(0).unsigned_abs()
    }
    pub fn minor_key(&self) -> bool {
        self.0[1] == 1
    }
}
