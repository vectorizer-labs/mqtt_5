use packattack::*;
use crate::error::MQTTParserError;

#[derive(Clone, Copy, Debug, PartialEq, FromBitReader)]
#[size_in_bits = "u8"]
#[repr(u8)]
pub enum ReasonCode
{
    Success = 0,
    //NormalDisconnection = 0,
    //GrantedQoS0 = 0,
    GrantedQoS1 = 1,
    GrantedQoS2 = 2,
    DisconnectWithWillMessage = 4,
    NoMatchingSubscribers = 16,
    NoSubscriptionExisted = 17,
    ContinueAuthentication = 24,
    ReAuthenticate = 25,
    UnspecifiedError = 128,
    MalformedPacket = 129,
    ProtocolError = 130,
    ImplementationSpecificError = 131,
    UnsupportedProtocolVersion = 132,
    ClientIdentifierNotValid = 133,
    BadUserNameorPassword = 134,
    NotAuthorized = 135,
    ServerUnavailable = 136,
    ServerBusy = 137,
    Banned = 138,
    ServerShuttingDown = 139,
    BadAuthenticationMethod = 140,
    KeepAliveTimeout = 141,
    SessionTakenOver = 142,
    TopicFilterInvalid = 143,
    TopicNameInvalid = 144,
    PacketIdentifierInUse = 145,
    PacketIdentifierNotFound = 146,
    RecieveMaximumExceeded = 147,
    TopicAliasInvalid = 148,
    PacketTooLarge = 149,
    MessageRateTooHigh = 150,
    QuotaExceeded = 151,
    AdministrativeAction = 152,
    PayloadFormatInvalid = 153,
    RetainNotSupported = 154,
    QoSNotSupported = 155,
    UseAnotherServer = 156,
    ServerMoved = 157,
    SharedSubscriptionsNotSupported = 158,
    ConnectionRateExceeded = 159,
    MaximumConnectTime = 160,
    SubscriptionIdentifiersNotSupported = 161,
    WildcardSubscriptionsNotSupported = 162
}

#[async_trait]
impl<R> FromBitReaderWithLength<MQTTParserError, R> for Vec<ReasonCode> where R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader_with_length(reader : &mut bitreader_async::BitReader<R>, len : usize) -> Result<Vec<ReasonCode>, MQTTParserError>
    {
        let mut reasons : Vec<ReasonCode> = Vec::new();

        let end = reader.byte_count() + len;

        while reader.byte_count() < end
        {
            reasons.push(ReasonCode::from_bitreader(reader).await?);
        }

        Ok(reasons)
    }
}