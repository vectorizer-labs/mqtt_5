//TODO: Expand this definition to more errors
/// Result type for those BitReader operations that can fail.
pub type Result<T> = std::result::Result<T, MQTTParserError>;

#[derive(Fail, Debug)]
pub enum MQTTParserError
{
    #[fail(display = "The VariableByteInteger could not be parsed because a malformed byte was read.")]
    MalformedVariableIntegerError,
    #[fail(display = "BitReaderError : {}", _0)]
    BitReaderError(#[cause] bitreader_async::error::BitReaderError),
    #[fail(display = "FromUtf8Error : {}", _0)]
    FromUtf8Error(#[cause] std::string::FromUtf8Error)
}

impl From<bitreader_async::error::BitReaderError> for MQTTParserError
{
    fn from(error : bitreader_async::error::BitReaderError) -> MQTTParserError
    {
        MQTTParserError::BitReaderError(error)
    }
}

impl From<std::string::FromUtf8Error> for MQTTParserError
{
    fn from(error : std::string::FromUtf8Error) -> MQTTParserError
    {
        MQTTParserError::FromUtf8Error(error)
    }
}
