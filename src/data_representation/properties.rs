use super::FromBitReader;
use super::super::error::Result;
use async_std::io::Read;
use async_trait::async_trait;

use super::VariableByteInteger;

#[derive(Clone, Debug, PartialEq, FromBitReader)]
#[size_in_bits = "VariableByteInteger"]
#[repr(u8)]
pub enum Property
{
    PayloadFormatIndicator(u8) = 1,
    MessageExpiryInterval(u32) = 2,
    ContentType(String) = 3,
    ResponseTopic(String) = 8,
    CorrelationData(Vec<u8>) = 9,
    SubscriptionIdentifier(u32) = 11,
    SessionExpiryInterval(u32) = 17,
    AssignedClientIdentifer(String) = 18,
    ServerKeepAlive(u16) = 19,
    AuthenticationMethod(String) = 21,
    AuthenticationData(Vec<u8>) = 22,
    RequestInformation(u8) = 23,
    WillDelayInterval(u32) = 24,
    RequestResponseInformation(u8) = 25,
    ResponseInformation(String) = 26,
    ServerReference(String) = 28,
    ReasonString(String) = 31,
    RecieveMaximum(u16) = 33,
    TopicAliasMaximum(u16) = 34,
    TopicAlias(u16) = 35,
    MaximumQoS(u8) = 36,
    RetainAvailible(u8) = 37,
    UserProperty(String) = 38,
    MaximumPacketSize(u32) = 39,
    WildcardSubscriptionAvailible(u8) = 40,
    SubscriptionIdentifierAvailible(u8) = 41,
    SharedSubscriptionAvailible(u8) = 42
}

pub type Properties = Vec<Property>;

#[async_trait]
impl<R> FromBitReader<R> for Properties where Self : Sized, R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader(reader : &mut bitreader_async::BitReader<R>) -> Result<Properties>
    {
        let length = super::VariableByteInteger::from_bitreader(reader).await?;

        let mut props : Vec<Property> = Vec::new();

        let end = reader.byte_count() + usize::from(length.clone()); 

        //println!("length : {}, start : {}, end : {}", usize::from(length), reader.byte_count(), end);

        //TODO: figure out a way to read the length remaining
        while reader.byte_count() < end
        {
            props.push(Property::from_bitreader(reader).await?);
        }

        Ok(props)
    }
}