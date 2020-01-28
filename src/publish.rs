use super::data_representation::{
    BinaryData,
    properties::Properties, 
    reserved_flags::ReservedFlags, 
    RemainingLength,
    UTF8EncodedString,
    qos::QoS
};

use packattack::*;

#[derive(Clone, Debug, PartialEq, FromReader)]
pub struct Publish
(
    #[hint] 
    RemainingLength,
    TopicName, 
    #[flag = "r_flags.qos != QoS::AtMostOnce"] 
    #[from_bytes] 
    Option<PacketIdentifier>,
    Properties, 
    #[payload] 
    Payload
);

pub type TopicName = UTF8EncodedString;
pub type PacketIdentifier = u16;
pub type Payload = BinaryData;