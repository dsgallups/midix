use std::io::Write;

use crate::prelude::*;

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
    /// All sound should turn off, but not immediately.
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
                byte_1: DataByte::new(other)?,
                byte_2: reader.read_next_as_databyte()?,
            },
        };
        Ok(controller)
    }
    /// Converts self to a vector of bytes.
    pub fn to_raw(&self) -> Vec<u8> {
        use Controller::*;
        match self {
            BankSelection(byte) => {
                vec![0x00, byte.value()]
            }
            ModulationCoarse(b) => {
                vec![0x01, b.value()]
            }
            ModulationFine(b) => {
                vec![0x21, b.value()]
            }
            DataEntryCoarse(b) => {
                vec![0x06, b.value()]
            }
            DataEntryFine(b) => {
                vec![0x26, b.value()]
            }
            VolumeCoarse(b) => {
                vec![0x07, b.value()]
            }
            VolumeFine(b) => {
                vec![0x27, b.value()]
            }
            PanCoarse(b) => {
                vec![0x0A, b.value()]
            }
            PanFine(b) => {
                vec![0x2A, b.value()]
            }
            ExpressionCoarse(b) => {
                vec![0x0B, b.value()]
            }
            ExpressionFine(b) => {
                vec![0x2B, b.value()]
            }
            HoldPedal(b) => {
                vec![0x40, b.value()]
            }
            ReverbSend(b) => {
                vec![0x5B, b.value()]
            }
            ChorusSend(b) => {
                vec![0x5D, b.value()]
            }
            NRPNCoarse => {
                vec![0x63]
            }
            NRPNFine => {
                vec![0x62]
            }
            SetNRPNCoarse(b) => {
                vec![0x65, b.value()]
            }
            SetNRPNFine(b) => {
                vec![0x64, b.value()]
            }
            MuteImmediately => {
                vec![0x78]
            }
            ResetAllControllers => {
                vec![0x79]
            }
            Mute => {
                vec![0x7B]
            }
            Other { byte_1, byte_2 } => {
                vec![byte_1.value(), byte_2.value()]
            }
        }
    }

    /// Converts self to a vector of bytes.
    pub fn write_bytes<W: Write>(&self, mut writer: W) -> io::Result<()> {
        use Controller::*;
        match self {
            BankSelection(byte) => {
                writer.write_all(&[0x00, byte.value()])?;
            }
            ModulationCoarse(b) => {
                writer.write_all(&[0x01, b.value()])?;
            }
            ModulationFine(b) => {
                writer.write_all(&[0x21, b.value()])?;
            }
            DataEntryCoarse(b) => {
                writer.write_all(&[0x06, b.value()])?;
            }
            DataEntryFine(b) => {
                writer.write_all(&[0x26, b.value()])?;
            }
            VolumeCoarse(b) => {
                writer.write_all(&[0x07, b.value()])?;
            }
            VolumeFine(b) => {
                writer.write_all(&[0x27, b.value()])?;
            }
            PanCoarse(b) => {
                writer.write_all(&[0x0A, b.value()])?;
            }
            PanFine(b) => {
                writer.write_all(&[0x2A, b.value()])?;
            }
            ExpressionCoarse(b) => {
                writer.write_all(&[0x0B, b.value()])?;
            }
            ExpressionFine(b) => {
                writer.write_all(&[0x2B, b.value()])?;
            }
            HoldPedal(b) => {
                writer.write_all(&[0x40, b.value()])?;
            }
            ReverbSend(b) => {
                writer.write_all(&[0x5B, b.value()])?;
            }
            ChorusSend(b) => {
                writer.write_all(&[0x5D, b.value()])?;
            }
            NRPNCoarse => {
                writer.write_all(&[0x63])?;
            }
            NRPNFine => {
                writer.write_all(&[0x62])?;
            }
            SetNRPNCoarse(b) => {
                writer.write_all(&[0x65, b.value()])?;
            }
            SetNRPNFine(b) => {
                writer.write_all(&[0x64, b.value()])?;
            }
            MuteImmediately => {
                writer.write_all(&[0x78])?;
            }
            ResetAllControllers => {
                writer.write_all(&[0x79])?;
            }
            Mute => {
                writer.write_all(&[0x7B])?;
            }
            Other { byte_1, byte_2 } => {
                writer.write_all(&[byte_1.value(), byte_2.value()])?;
            }
        }
        Ok(())
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
