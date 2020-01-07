use super::data_representation::{
    reserved_flags::ReservedFlags,
    RemainingLength
};

use packattack::*;

#[derive(Clone, Debug, PartialEq, FromBitReader)]
pub struct Pingresp
(
    ReservedFlags,
    RemainingLength
);