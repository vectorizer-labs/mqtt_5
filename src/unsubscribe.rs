use super::data_representation::{
    properties::Properties, 
    reserved_flags::ReservedFlags,
    RemainingLength,
    TwoByteInteger,
    UTF8EncodedString
};

use packattack::*;
use crate::error::MQTTParserError;

#[derive(Clone, Debug, PartialEq, FromReader)]
pub struct Unsubscribe
(
    ReservedFlags,
    #[expose = "r_length"]
    RemainingLength,
    PacketIdentifier,
    Properties,
    //find the remaining length by subtracting the bytes we've already read from the total size
    //we subtract 1 byte for reserved flags and the length of the Remaining Length
    //since these are not included in the the remaining length measure
    #[length = "usize::from(&r_length) - (reader.byte_count()-1 - r_length.size())"]
    Vec<TopicFilter>,
);

pub type PacketIdentifier = TwoByteInteger;

pub type TopicFilter = UTF8EncodedString;

#[async_trait]
impl<R> FromReaderWithLength<MQTTParserError, R> for Vec<TopicFilter> where R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader_with_length(reader : &mut bitreader_async::BitReader<R>, len : usize) -> Result<Vec<TopicFilter>, MQTTParserError>
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