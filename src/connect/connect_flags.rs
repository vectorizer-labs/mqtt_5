use super::super::qos::QoS;
use std::error;
use std::fmt;

const USERNAMEFLAG_BIT : u8 = 7;
const PASSWORD_BIT : u8 = 6;
const WILLRETAIN_BIT : u8 = 5;
const WILLFLAG_BIT : u8 = 2;
const CLEANSTART_BIT : u8 = 1;
const RESERVED_BIT : u8 = 0;

#[derive(Debug, Clone, PartialEq)]
pub struct ConnectFlags
{
    UserNameFlag : bool,
    PasswordFlag : bool,
    WillRetain : bool,
    WillQoS : QoS,
    WillFlag : bool,
    CleanStart: bool
}


pub fn from_u8(byte: u8) -> Result<ConnectFlags, MalformedConnectFlagsError>
{
    match byte & (1 << RESERVED_BIT) !=0
    {
        false => 
        {

            let raw_qos = byte & 0b0110u8; 

            Ok(ConnectFlags
            {
                UserNameFlag : byte & (1 << USERNAMEFLAG_BIT) != 0,
                PasswordFlag : byte & (1 << PASSWORD_BIT) != 0,
                WillRetain : byte & (1 << WILLRETAIN_BIT) != 0,
                WillQoS : QoS::from_u8(raw_qos),
                WillFlag : byte & (1 << WILLFLAG_BIT) != 0,
                CleanStart : byte & (1 << CLEANSTART_BIT) != 0
            })
        }
        true => Err(MalformedConnectFlagsError)
    }
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