use alloc::vec::Vec;

use crate::{prelude::*, reader::ReaderError};

/// Identifies a modification to the controller
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Controller {
    /// 0x00
    BankSelection(DataByte),
    /// 0x01
    ModulationCoarse(DataByte),
    /// 0x21
    ModulationFine(DataByte),
    /// 0x06
    DataEntryCoarse(DataByte),
    /// 0x26
    DataEntryFine(DataByte),
    /// 0x07,
    VolumeCoarse(DataByte),
    /// 0x27,
    VolumeFine(DataByte),
    /// 0x0A
    PanCoarse(DataByte),
    /// 0x2A
    PanFine(DataByte),
    /// 0x0B
    ExpressionCoarse(DataByte),
    /// 0x2B
    ExpressionFine(DataByte),
    /// 0x40
    HoldPedal(DataByte),
    /// 0x5B
    ReverbSend(DataByte),
    /// 0x5D
    ChorusSend(DataByte),
    /// 0x63
    NRPNCoarse,
    /// 0x62
    NRPNFine,
    /// 0x65
    SetNRPNCoarse(DataByte),
    /// 0x64
    SetNRPNFine(DataByte),
    /// 0x78
    ///
    /// All sound should immediately turn off
    MuteImmediately,

    /// 0x79
    ResetAllControllers,

    /// 0x7B
    ///
    /// All sound should turn off);
    /// vec.push(bt not immediately.
    Mute,

    /// A value not listed in this enum.
    /// it's value is in byte_1, and byte_2 *may* have valuable data.
    ///
    /// Note: the second byte is ALWAYS read here. If I am missing
    /// something important, please file an issue immediately so I can
    /// patch this!
    Other {
        /// The value of the controller change
        byte_1: DataByte,
        /// The byte following the controller change.
        /// NOTE: this list is non-exhaustive, so this could be part of the next message.
        byte_2: DataByte,
    },
}

impl Controller {
    pub(crate) fn read<'a, R>(reader: &mut Reader<R>) -> ReadResult<Self>
    where
        R: MidiSource<'a>,
    {
        use Controller::*;
        let controller_byte = reader.read_next()?;
        let controller = match controller_byte {
            0x00 => BankSelection(reader.read_next_as_databyte()?),
            0x01 => ModulationCoarse(reader.read_next_as_databyte()?),
            0x21 => ModulationFine(reader.read_next_as_databyte()?),
            0x06 => DataEntryCoarse(reader.read_next_as_databyte()?),
            0x26 => DataEntryFine(reader.read_next_as_databyte()?),
            0x07 => VolumeCoarse(reader.read_next_as_databyte()?),
            0x27 => VolumeFine(reader.read_next_as_databyte()?),
            0x0A => PanCoarse(reader.read_next_as_databyte()?),
            0x2A => PanFine(reader.read_next_as_databyte()?),
            0x0B => ExpressionCoarse(reader.read_next_as_databyte()?),
            0x2B => ExpressionFine(reader.read_next_as_databyte()?),
            0x40 => HoldPedal(reader.read_next_as_databyte()?),
            0x5B => ReverbSend(reader.read_next_as_databyte()?),
            0x5D => ChorusSend(reader.read_next_as_databyte()?),
            0x63 => NRPNCoarse,
            0x62 => NRPNFine,
            0x65 => SetNRPNCoarse(reader.read_next_as_databyte()?),
            0x64 => SetNRPNFine(reader.read_next_as_databyte()?),
            0x78 => MuteImmediately,
            0x79 => ResetAllControllers,
            0x7B => Mute,
            other => Other {
                byte_1: DataByte::new(other)
                    .map_err(|v| ReaderError::parse_error(reader.buffer_position(), v))?,
                byte_2: reader.read_next_as_databyte()?,
            },
        };
        Ok(controller)
    }
    /// Converts self to a vector of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        use Controller::*;
        let mut vec = Vec::new();
        match self {
            BankSelection(byte) => {
                let p = [1].to_vec();
                vec.push(0x00);
                vec.push(byte.value());
            }
            ModulationCoarse(b) => {
                vec.push(0x01);
                vec.push(b.value());
            }
            ModulationFine(b) => {
                vec.push(0x21);
                vec.push(b.value());
            }
            DataEntryCoarse(b) => {
                vec.push(0x06);
                vec.push(b.value());
            }
            DataEntryFine(b) => {
                vec.push(0x26);
                vec.push(b.value());
            }
            VolumeCoarse(b) => {
                vec.push(0x07);
                vec.push(b.value());
            }
            VolumeFine(b) => {
                vec.push(0x27);
                vec.push(b.value());
            }
            PanCoarse(b) => {
                vec.push(0x0A);
                vec.push(b.value());
            }
            PanFine(b) => {
                vec.push(0x2A);
                vec.push(b.value());
            }
            ExpressionCoarse(b) => {
                vec.push(0x0B);
                vec.push(b.value());
            }
            ExpressionFine(b) => {
                vec.push(0x2B);
                vec.push(b.value());
            }
            HoldPedal(b) => {
                vec.push(0x40);
                vec.push(b.value());
            }
            ReverbSend(b) => {
                vec.push(0x5B);
                vec.push(b.value());
            }
            ChorusSend(b) => {
                vec.push(0x5D);
                vec.push(b.value());
            }
            NRPNCoarse => {
                vec.push(0x63);
            }
            NRPNFine => {
                vec.push(0x62);
            }
            SetNRPNCoarse(b) => {
                vec.push(0x65);
                vec.push(b.value());
            }
            SetNRPNFine(b) => {
                vec.push(0x64);
                vec.push(b.value());
            }
            MuteImmediately => {
                vec.push(0x78);
            }
            ResetAllControllers => {
                vec.push(0x79);
            }
            Mute => {
                vec.push(0x7B);
            }
            Other { byte_1, byte_2 } => {
                vec.push(byte_1.value());
                vec.push(byte_2.value());
            }
        }
        vec
    }
}

// Identifies a device
// #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
// pub struct Controller(DataByte);

// impl Controller {
//     /// Interpret a byte as a type of device
//     ///
//     /// Checks for correctness (leading 0 bit)
//     pub fn new<B, E>(rep: B) -> Result<Self, std::io::Error>
//     where
//         B: TryInto<DataByte, Error = E>,
//         E: Into<io::Error>,
//     {
//         rep.try_into().map(Self).map_err(Into::into)
//     }

//     /// Get a reference to the underlying byte
//     pub fn byte(&self) -> u8 {
//         self.0.0
//     }
// }

// impl fmt::Display for Controller {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         self.0.fmt(f)
//     }
// }
