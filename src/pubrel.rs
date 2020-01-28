use super::data_representation::{
    properties::Properties,
    reason_code::ReasonCode,
    RemainingLength
};

use packattack::*;

#[derive(Clone, Debug, PartialEq, FromReader)]
pub struct Pubrel
(
    RemainingLength,
    #[from_bytes] PacketIdentifier,
    #[from_bytes] ReasonCode,
    Properties
);

pub type PacketIdentifier = u16;