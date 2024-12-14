/// Contains traits that can represent structs

pub trait MidiBits {
    type BitRepresentation;
    fn as_bits(&self) -> Self::BitRepresentation;
    fn from_bits(rep: Self::BitRepresentation) -> Result<Self, std::io::Error>
    where
        Self: Sized;
}

pub trait FromMidiBytes {
    fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error>
    where
        Self: Sized;
}
pub trait FromMidiBytesOwned {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, std::io::Error>
    where
        Self: Sized;
}

pub trait AsMidiBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

pub trait AsBorrowedBytes {
    fn borrowed_bytes(&self) -> &[u8];
}
