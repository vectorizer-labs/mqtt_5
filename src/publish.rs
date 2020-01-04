use super::data_representation::{
    BinaryData,
    properties::Properties, 
    reserved_flags::ReservedFlags, 
    RemainingLength,
    FromBitReader,
    UTF8EncodedString,
    TwoByteInteger,
    qos::QoS
};


use super::error::Result;
use async_std::io::Read;

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
    #[length = "usize::from(r_length) - reader.byte_count()"]
    Payload

);

pub type TopicName = UTF8EncodedString;
pub type PacketIdentifier = TwoByteInteger;
pub type Payload = BinaryData;