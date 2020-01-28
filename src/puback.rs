use super::data_representation::{
    properties::Properties,
    reason_code::ReasonCode,
    RemainingLength,
    TwoByteInteger
};

use packattack::*;

#[derive(Clone, Debug, PartialEq, FromReader)]
pub struct Puback
(
    RemainingLength,
    #[from_bytes] PacketIdentifier,
    #[from_bytes] ReasonCode,
    Properties
);

pub type PacketIdentifier = u16;