use super::data_representation::{
    properties::Properties, 
    reserved_flags::ReservedFlags, 
    reason_code::ReasonCode,
    RemainingLength,
    TwoByteInteger,
    UTF8EncodedString,
    qos::QoS
};

use packattack::*;
use super::error::MQTTParserError;

#[derive(Clone, Debug, PartialEq, FromBitReader)]
pub struct Subscribe
(
    ReservedFlags,
    #[expose = "r_length"]
    RemainingLength,
    PacketIdentifier,
    ReasonCode,
    Properties,
    //find the remaining length by subtracting the bytes we've already read from the total size
    //we subtract 1 byte for reserved flags and the length of the Remaining Length
    //since these are not included in the the remaining length measure
    #[length = "usize::from(&r_length) - (reader.byte_count()-1 - r_length.size())"]
    TopicFilterList
);

pub type PacketIdentifier = TwoByteInteger;

#[derive(Clone, Debug, PartialEq, FromBitReader)]
pub struct TopicFilter(UTF8EncodedString, SubscriptionOptions);

#[derive(Clone, Debug, PartialEq, FromBitReader)]
pub struct SubscriptionOptions
{
    maximum_qos : QoS,
    no_local : bool,
    retain_as_publish : bool,
    retain_handling : RetainHandling,
    reserved : ReservedSubscriptionBits
}

#[derive(Clone, Debug, PartialEq, FromBitReader)]
#[size_in_bits = 2]
pub enum ReservedSubscriptionBits
{
    Clear = 0
}

#[derive(Clone, Debug, PartialEq, FromBitReader)]
#[size_in_bits = 2]
pub enum RetainHandling
{
    SendRetainedMessagesOnSubscribe = 0,
    SendRetainedMessagesOnSubscribeIfSubscriptionDidntExist = 1,
    DoNotSendRetainedMessagesOnSubscribe = 2
}

pub type TopicFilterList = Vec<TopicFilter>; 


#[async_trait]
impl<R> FromBitReaderWithLength<MQTTParserError, R> for TopicFilterList where R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader_with_length(reader : &mut bitreader_async::BitReader<R>, len : usize) -> Result<TopicFilterList, MQTTParserError>
    {
        let mut topics : Vec<TopicFilter> = Vec::new();

        let end = reader.byte_count() + len;

        while reader.byte_count() < end
        {
            topics.push(TopicFilter::from_bitreader(reader).await?);
        }

        Ok(topics)
    }
}