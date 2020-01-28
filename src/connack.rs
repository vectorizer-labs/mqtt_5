use super::data_representation::{
    reason_code::ReasonCode,
    properties::Properties, 
    reserved_flags::ReservedFlags, 
    RemainingLength
};
use packattack::*;

#[derive(Clone, Debug, PartialEq, FromReader)]
pub struct Connack
(
    RemainingLength,
    #[from_bytes] ConnackFlags,
    #[from_bytes] ReasonCode,
    Properties
);

//these flags should always be 0 so if we don't get a match then its an MQTT Error
#[derive(Clone, Debug, PartialEq, FromBytes)]
#[size_in_bits = 7]
#[repr(u8)]
pub enum FlagsReserved
{
    Clear = 0
}

#[derive(Clone, Debug, PartialEq, FromBytes)]
pub struct ConnackFlags
{
    session_pressent : bool,
    reserved : FlagsReserved // read the last 7 bits and make sure they're 0
}