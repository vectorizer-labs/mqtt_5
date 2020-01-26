use packattack::*;

/// Result type for those BitReader operations that can fail.
pub type Result<T> = std::result::Result<T, MQTTParserError>;

#[derive(Fail, Debug)]
pub enum MQTTParserError
{
    #[fail(display = "The VariableByteInteger could not be parsed because a malformed byte was read.")]
    MalformedVariableIntegerError,
    #[fail(display = "FromUtf8Error : {}", _0)]
    FromUtf8Error(#[cause] std::string::FromUtf8Error),
    #[fail(display = "IO Error: {}", _0)]
    IO(#[cause] std::io::Error),
    #[fail(display = "PackattackError Error: {}", _0)]
    PackattackError(packattack::PackattackParserError),
    #[fail(display = "Non Error Type for non failable parsing: This should never be reached!")]
    None
}

impl From<std::string::FromUtf8Error> for MQTTParserError
{
    fn from(error : std::string::FromUtf8Error) -> MQTTParserError
    {
        MQTTParserError::FromUtf8Error(error)
    }
}

impl From<packattack::error::PackattackParserError> for MQTTParserError
{
    fn from(error : packattack::error::PackattackParserError) -> MQTTParserError
    {
        MQTTParserError::PackattackError(error)
    }
}

impl From<std::io::Error> for MQTTParserError
{
    fn from(error : std::io::Error) -> MQTTParserError
    {
        MQTTParserError::IO(error)
    }
}

impl From<()> for MQTTParserError
{
    fn from(_error : ()) -> MQTTParserError
    {
        MQTTParserError::None
    }
}
