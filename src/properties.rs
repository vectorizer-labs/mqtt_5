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

async fn read_property(mut reader: &mut BufReader<runtime::net::tcp::TcpStream>) -> Property
{
    let property_identifier : u32 = read_variable_byte_integer(&mut reader).await;
    return match property_identifier
    {
        1 => Property::PayloadFormatIndicator(read_byte(&mut reader).await),
        2 => Property::MessageExpiryInterval(read_four_byte_integer(&mut reader).await),
        3 => Property::ContentType(read_utf8_encoded_string(&mut reader).await),
        8 => Property::ResponseTopic(read_utf8_encoded_string(&mut reader).await),
        9 => Property::CorrelationData(read_binary_data(&mut reader).await),
        11 => Property::SubscriptionIdentifier(read_variable_byte_integer(&mut reader).await),
        17 => Property::SessionExpiryInterval(read_four_byte_integer(&mut reader).await),
        18 => Property::AssignedClientIdentifer(read_utf8_encoded_string(&mut reader).await),
        19 => Property::ServerKeepAlive(read_two_byte_integer(&mut reader).await),
        21 => Property::AuthenticationMethod(read_utf8_encoded_string(&mut reader).await),
        22 => Property::AuthenticationData(read_binary_data(&mut reader).await),
        23 => Property::RequestInformation(read_byte(&mut reader).await),
        24 => Property::WillDelayInterval(read_four_byte_integer(&mut reader).await),
        25 => Property::RequestResponseInformation(read_byte(&mut reader).await),
        26 => Property::ResponseInformation(read_utf8_encoded_string(&mut reader).await),
        28 => Property::ServerReference(read_utf8_encoded_string(&mut reader).await),
        31 => Property::ReasonString(read_utf8_encoded_string(&mut reader).await),
        33 => Property::RecieveMaximum(read_two_byte_integer(&mut reader).await),
        34 => Property::TopicAliasMaximum(read_two_byte_integer(&mut reader).await),
        35 => Property::TopicAlias(read_two_byte_integer(&mut reader).await),
        36 => Property::MaximumQoS(read_byte(&mut reader).await),
        37 => Property::RetainAvailible(read_byte(&mut reader).await),
        38 => Property::UserProperty(read_utf8_encoded_string(&mut reader).await),
        39 => Property::MaximumPacketSize(read_four_byte_integer(&mut reader).await),
        40 => Property::WildcardSubscriptionAvailible(read_byte(&mut reader).await),
        41 => Property::SubscriptionIdentifierAvailible(read_byte(&mut reader).await),
        42 => Property::SharedSubscriptionAvailible(read_byte(&mut reader).await),
        _ => panic!("Couldn't parse property! Malformed Packet!")
    };

}