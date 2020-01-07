use super::data_representation::{
    properties::Properties, 
    reserved_flags::ReservedFlags, 
    reason_code::ReasonCode,
    RemainingLength
};

use packattack::*;

#[derive(Clone, Debug, PartialEq, FromBitReader)]
pub struct Unsuback
(
    ReservedFlags,
    #[expose = "r_length"]
    RemainingLength,
    Properties,
    //find the remaining length by subtracting the bytes we've already read from the total size
    //we subtract 1 byte for reserved flags and the length of the Remaining Length
    //since these are not included in the the remaining length measure
    #[length = "usize::from(&r_length) - (reader.byte_count()-1 - r_length.size())"]
    Vec<ReasonCode>
);