use crate::prelude::*;
use core::fmt;

/// Identifies a modification to the controller
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
    /// it's value is in byte_1, and byte_2 *may* have data.
    Other {
        byte_1: DataByte,
        byte_2: Option<DataByte>,
    },
}

impl Controller {
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
            Other { byte_1, byte_2 } => match byte_2 {
                Some(b2) => {
                    vec![byte_1.value(), b2.value()]
                }
                None => {
                    vec![byte_1.value()]
                }
            },
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
