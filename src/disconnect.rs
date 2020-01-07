use super::data_representation::{
    reserved_flags::ReservedFlags,
    reason_code::ReasonCode,
    RemainingLength,
    properties::Properties
};

use packattack::*;

#[derive(Clone, Debug, PartialEq, FromBitReader)]
pub struct Disconnect
(
    ReservedFlags,
    #[expose = "r_length"]
    RemainingLength,
    #[flag = "usize::from(&r_length) > 0"]
    Option<ReasonCode>,
    #[flag = "usize::from(r_length) > 0"]
    Option<Properties>

);