pub type KeepAlive = u16;

use super::data_representation::UTF8EncodedString;

#[derive(Clone, Debug, PartialEq)]
pub enum Protocol
{
    MQTT,
    MQIsdp
}

impl FromBitReader for Protocol
{
    fn from_bitreader(reader : &mut BitReader) -> Result<Protocol>
    {

        let protocol = UTF8EncodedString::from_bitreader(reader).unwrap();

        match protocol.as_str()
        {
            "MQTT" => Ok(Protocol::MQTT),
            _ => panic!("Couldn't parse control packet because the client tried to use a protocol other than MQTT!")
        }
    }
}

pub type ProtocolLevel = u8;

use super::data_representation::{ FromBitReader , qos::QoS};

#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug, PartialEq, FromBitReader)]
pub struct ConnectFlags
{
    pub UserNameFlag : bool,
    pub PasswordFlag : bool,
    pub WillRetain : bool,
    pub WillQoS : QoS,
    pub WillFlag : bool,
    pub CleanStart: bool,
    pub Reserve : bool
    
}
/*
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
}*/

#[cfg(test)]
mod test
{
    use super::*;
    use bitreader::BitReader;

    #[test]
    fn build_connect_from_byte()
    {
        let byte : [u8;1] = [0b11110110];

        let mut reader = BitReader::new(&byte);

        let parsed_connect = ConnectFlags::from_bitreader(&mut reader).unwrap();

        println!("connect: {:#? } ", parsed_connect);

        assert_eq!(parsed_connect, ConnectFlags
        {
            UserNameFlag : true,
            PasswordFlag : true,
            WillRetain : true,
            WillQoS : super::super::data_representation::qos::QoS::ExactlyOnce,
            WillFlag : true,
            CleanStart : true,
            Reserve : false
        });
    }
}