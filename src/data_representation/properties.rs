use super::VariableByteInteger;

use packattack::*;
use super::super::error::MQTTParserError;

#[derive(Clone, Debug, PartialEq, FromReader)]
#[size_in_bits = "VariableByteInteger"]
#[repr(u8)]
pub enum Property
{
    PayloadFormatIndicator(#[from_bytes] u8) = 1,
    MessageExpiryInterval(#[from_bytes] u32) = 2,
    ContentType(String) = 3,
    ResponseTopic(String) = 8,
    CorrelationData(Vec<u8>) = 9,
    SubscriptionIdentifier(#[from_bytes] u32) = 11,
    SessionExpiryInterval(#[from_bytes] u32) = 17,
    AssignedClientIdentifer(String) = 18,
    ServerKeepAlive(#[from_bytes] u16) = 19,
    AuthenticationMethod(String) = 21,
    AuthenticationData(Vec<u8>) = 22,
    RequestInformation(#[from_bytes] u8) = 23,
    WillDelayInterval(#[from_bytes] u32) = 24,
    RequestResponseInformation(#[from_bytes] u8) = 25,
    ResponseInformation(String) = 26,
    ServerReference(String) = 28,
    ReasonString(String) = 31,
    RecieveMaximum(#[from_bytes] u16) = 33,
    TopicAliasMaximum(#[from_bytes] u16) = 34,
    TopicAlias(#[from_bytes] u16) = 35,
    MaximumQoS(#[from_bytes] u8) = 36,
    RetainAvailible(#[from_bytes] u8) = 37,
    UserProperty(String) = 38,
    MaximumPacketSize(#[from_bytes] u32) = 39,
    WildcardSubscriptionAvailible(#[from_bytes] u8) = 40,
    SubscriptionIdentifierAvailible(#[from_bytes] u8) = 41,
    SharedSubscriptionAvailible(#[from_bytes] u8) = 42
}

pub type Properties = Vec<Property>;

#[async_trait]
impl<R> FromReader<MQTTParserError, R> for Properties where Self : Sized, R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_reader(reader : &mut R) -> Result<Properties, MQTTParserError>
    {

        let length : usize = usize::from(<VariableByteInteger>::from_reader(reader).await?);

        let mut buffer : Vec<u8> = vec![0; length];

        reader.read_exact(&mut buffer).await?;

        let mut slice : &[u8] = buffer.as_slice();

        let mut props : Vec<Property> = Vec::new();

        //reading from a slice updates the remaining length
        //so we can just check the len after each read
        while slice.len() > 0
        {
            props.push(Property::from_reader(&mut slice).await?);
        }

        Ok(props)
    }
}