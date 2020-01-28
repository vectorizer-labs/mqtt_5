use super::data_representation::{
    properties::Properties, 
    reason_code::ReasonCode,
    RemainingLength,
    TwoByteInteger,
    UTF8EncodedString,
    qos::QoS
};

use packattack::*;
use super::error::MQTTParserError;

#[derive(Clone, Debug, PartialEq, FromReader)]
#[hint = "RemainingLength"]
pub struct Subscribe
(
    #[from_bytes] PacketIdentifier,
    #[from_bytes] ReasonCode,
    Properties,
    //find the remaining length by subtracting the bytes we've already read from the total size
    //we subtract 1 byte for reserved flags and the length of the Remaining Length
    //since these are not included in the the remaining length measure
    #[payload]
    TopicFilterList
);

pub type PacketIdentifier = TwoByteInteger;

#[derive(Clone, Debug, PartialEq, FromReader)]
pub struct TopicFilter(
    UTF8EncodedString, 
    #[from_bytes] SubscriptionOptions
);

#[derive(Clone, Debug, PartialEq, FromBytes)]
pub struct SubscriptionOptions
{
    maximum_qos : QoS,
    no_local : bool,
    retain_as_publish : bool,
    retain_handling : RetainHandling,
    reserved : ReservedSubscriptionBits
}

#[derive(Clone, Debug, PartialEq, FromBytes)]
#[from_bits]
#[size_in_bits = 2]
pub enum ReservedSubscriptionBits
{
    Clear = 0
}

#[derive(Clone, Debug, PartialEq, FromBytes)]
#[from_bits]
#[size_in_bits = 2]
pub enum RetainHandling
{
    SendRetainedMessagesOnSubscribe = 0,
    SendRetainedMessagesOnSubscribeIfSubscriptionDidntExist = 1,
    DoNotSendRetainedMessagesOnSubscribe = 2
}

pub type TopicFilterList = Vec<TopicFilter>; 

/*
impl Bitsize for TopicFilterList 
{
    const SIZE_IN_BITS : usize = 0; 
}

impl FromBytes<(), &mut &[u8]> for TopicFilterList where Self : Sized
{
    #[inline]
    fn from_bytes(bytes : &mut &[u8]) -> std::result::Result<TopicFilterList,()>
    {
        let mut topics : Vec<TopicFilter> = Vec::new();

        while bytes.len() > 0
        {
            topics.push(TopicFilter::from_reader(bytes)?);
        }

        Ok(topics)
    }
}*/