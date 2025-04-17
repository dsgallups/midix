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
    NRPNCoarse(DataByte),
    /// 0x62
    NRPNFine(DataByte),
    /// 0x65
    SetNRPNCoarse(DataByte),
    /// 0x64
    SetNRPNFine(DataByte),
    /// 0x78
    ///
    /// All sound should immediately turn off
    MuteImmediately(DataByte),

    /// 0x79
    ResetAllControllers(DataByte),

    /// 0x7B
    ///
    /// All sound should turn off);
    /// [bt not immediatel]
    Mute(DataByte),

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
        let data_byte = reader.read_next_as_databyte()?;
        let controller = match controller_byte {
            0x00 => BankSelection(data_byte),
            0x01 => ModulationCoarse(data_byte),
            0x21 => ModulationFine(data_byte),
            0x06 => DataEntryCoarse(data_byte),
            0x26 => DataEntryFine(data_byte),
            0x07 => VolumeCoarse(data_byte),
            0x27 => VolumeFine(data_byte),
            0x0A => PanCoarse(data_byte),
            0x2A => PanFine(data_byte),
            0x0B => ExpressionCoarse(data_byte),
            0x2B => ExpressionFine(data_byte),
            0x40 => HoldPedal(data_byte),
            0x5B => ReverbSend(data_byte),
            0x5D => ChorusSend(data_byte),
            0x63 => NRPNCoarse(data_byte),
            0x62 => NRPNFine(data_byte),
            0x65 => SetNRPNCoarse(data_byte),
            0x64 => SetNRPNFine(data_byte),
            0x78 => MuteImmediately(data_byte),
            0x79 => ResetAllControllers(data_byte),
            0x7B => Mute(data_byte),
            other => Other {
                byte_1: DataByte::new(other)
                    .map_err(|v| ReaderError::parse_error(reader.buffer_position(), v))?,
                byte_2: data_byte,
            },
        };
        Ok(controller)
    }
    /// Converts self to a vector of bytes.
    pub fn to_bytes(&self) -> [u8; 2] {
        use Controller::*;
        match self {
            BankSelection(byte) => [0x00, byte.value()],
            ModulationCoarse(b) => [0x01, b.value()],
            ModulationFine(b) => [0x21, b.value()],
            DataEntryCoarse(b) => [0x06, b.value()],
            DataEntryFine(b) => [0x26, b.value()],
            VolumeCoarse(b) => [0x07, b.value()],
            VolumeFine(b) => [0x27, b.value()],
            PanCoarse(b) => [0x0A, b.value()],
            PanFine(b) => [0x2A, b.value()],
            ExpressionCoarse(b) => [0x0B, b.value()],
            ExpressionFine(b) => [0x2B, b.value()],
            HoldPedal(b) => [0x40, b.value()],
            ReverbSend(b) => [0x5B, b.value()],
            ChorusSend(b) => [0x5D, b.value()],
            NRPNCoarse(b) => [0x63, b.value()],
            NRPNFine(b) => [0x62, b.value()],
            SetNRPNCoarse(b) => [0x65, b.value()],
            SetNRPNFine(b) => [0x64, b.value()],
            MuteImmediately(b) => [0x78, b.value()],
            ResetAllControllers(b) => [0x79, b.value()],
            Mute(b) => [0x7B, b.value()],
            Other { byte_1, byte_2 } => [byte_1.value(), byte_2.value()],
        }
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
