use super::FromBitReader;

#[derive(Clone, Debug, PartialEq, FromBitReader)]
#[size_in_bits = 8]
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

/*
impl FromBitReader for Properties
{
    fn from_bitreader(reader : &mut BitReader) -> Result<Properties>
    {
        let length = super::VariableByteInteger::from_bitreader(reader)?;

        //TODO get the index from bitreader

        //while(end > bitreader.index)
        //{ read Properties }
    }

}*/