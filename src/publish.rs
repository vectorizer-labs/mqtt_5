use super::data_representation::{
    BinaryData,
    properties::Properties, 
    reserved_flags::ReservedFlags, 
    RemainingLength,
    UTF8EncodedString,
    TwoByteInteger,
    qos::QoS
};

use packattack::*;

#[derive(Clone, Debug, PartialEq, FromBitReader)]
pub struct Publish
(
    #[expose = "r_flags"]
    ReservedFlags,
    #[expose = "r_length"]
    RemainingLength,
    TopicName,
    #[flag = "r_flags.qos != QoS::AtMostOnce"]
    Option<PacketIdentifier>,
    Properties,
    //find the remaining length by subtracting the bytes we've already read from the total size
    //we subtract 1 byte for reserved flags and the length of the Remaining Length
    //since these are not included in the the remaining length measure
    #[length = "usize::from(&r_length) - (reader.byte_count()-1 - r_length.size())"]
    Payload
);

pub type TopicName = UTF8EncodedString;
pub type PacketIdentifier = TwoByteInteger;
pub type Payload = BinaryData;