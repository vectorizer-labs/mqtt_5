pub type KeepAlive = u16;

pub type Protocol = String;

/*pub enum Protocol
{
    MQTT,
    MQIsdp
}

impl Protocol
{
    pub fn from_string(protocol : &str) -> Protocol
    {
        match protocol 
        {
            "MQTT" => Protocol::MQTT,
            _ => panic!("Couldn't parse control packet because the client tried to use a protocol other than MQTT!")
        }
    }
}*/

pub type ProtocolLevel = u8;

use super::qos::QoS;
use std::error;
use std::fmt;

use packattack::FromBytes;
use packattack::FromByte;

#[derive(Clone, Copy, Debug, PartialEq, FromBytes)]
pub struct ConnectFlags
{
    #[start_byte]
    UserNameFlag : bool,
    PasswordFlag : bool,
    WillRetain : bool,
    WillQoS : QoS,
    WillFlag : bool,
    #[end_byte]
    CleanStart: bool
    
}

#[derive(Debug, Clone)]
pub struct MalformedConnectFlagsError;

impl fmt::Display for MalformedConnectFlagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The packet was malformed! Reserved Flag in CONNECT Packet was set to 1.")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for MalformedConnectFlagsError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
