use packattack::*;
use crate::error::MQTTParserError;

#[derive(Clone, Copy, Debug, PartialEq, FromBytes)]
#[size_in_bits = 8]
#[from_bytes]
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

/*
#[async_trait]
impl<R> FromReader<MQTTParserError, R> for Vec<ReasonCode> where R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_reader(reader : &mut R) -> Result<Vec<ReasonCode>, MQTTParserError>
    {
        let length : usize = usize::from(<VariableByteInteger>::from_reader(reader).await?);

        let mut buffer : Vec<u8> = vec![0; length];

        reader.read_exact(&mut buffer).await?;

        let mut slice : &[u8] = buffer.as_slice();

        let mut reason_codes : Vec<ReasonCode> = Vec::new();

        //reading from a slice updates the remaining length
        //so we can just check the len after each read
        while slice.len() > 0
        {
            reason_codes.push(ReasonCode::from_reader(&mut slice).await?);
        }

        Ok(reason_codes)
    }
}*/