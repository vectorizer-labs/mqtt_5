use super::low_level_read::*;
use futures::io::{BufReader};

pub enum Property
{
    PayloadFormatIndicator(u8),
    MessageExpiryInterval(u32),
    ContentType(String),
    ResponseTopic(String),
    CorrelationData(Vec<u8>),
    SubscriptionIdentifier(u32),
    SessionExpiryInterval(u32),
    AssignedClientIdentifer(String),
    ServerKeepAlive(u16),
    AuthenticationMethod(String),
    AuthenticationData(Vec<u8>),
    RequestInformation(u8),
    WillDelayInterval(u32),
    RequestResponseInformation(u8),
    ResponseInformation(String),
    ServerReference(String),
    ReasonString(String),
    RecieveMaximum(u16),
    TopicAliasMaximum(u16),
    TopicAlias(u16),
    MaximumQoS(u8),
    RetainAvailible(u8),
    UserProperty(String),
    MaximumPacketSize(u32),
    WildcardSubscriptionAvailible(u8),
    SubscriptionIdentifierAvailible(u8),
    SharedSubscriptionAvailible(u8)
}