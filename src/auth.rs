use super::data_representation::{
    properties::Properties, 
    reserved_flags::ReservedFlags, 
    reason_code::ReasonCode,
    RemainingLength
};

use packattack::*;

#[derive(Clone, Debug, PartialEq, FromBitReader)]
pub struct Auth
(
    ReservedFlags,
    RemainingLength,
    ReasonCode,
    Properties
);