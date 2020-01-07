use super::data_representation::{
    properties::Properties, 
    reserved_flags::ReservedFlags, 
    reason_code::ReasonCode,
    RemainingLength,
    TwoByteInteger
};

use packattack::*;

#[derive(Clone, Debug, PartialEq, FromBitReader)]
pub struct Pubrel
(
    ReservedFlags,
    RemainingLength,
    PacketIdentifier,
    ReasonCode,
    Properties
);

pub type PacketIdentifier = TwoByteInteger;