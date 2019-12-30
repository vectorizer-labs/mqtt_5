use super::data_representation::{
    properties::Properties, 
    qos::QoS,
    reserved_flags::ReservedFlags, 
    UTF8EncodedString, 
    FromBitReader,
    BinaryData
};

use super::error::Result;
use super::RemainingLength;
use async_trait::async_trait;
use async_std::io::Read;

#[derive(Clone, Debug, PartialEq, FromBitReader)]
pub struct Connect
(
    ReservedFlags, 
    RemainingLength, 
    Protocol, 
    ProtocolLevel,
    #[expose = "c_flags"] 
    ConnectFlags, 
    KeepAlive,
    Properties
    /*ClientID,
    #[flag = "c_flags.WillFlag"]
    Option<WillProperties>,
    #[flag = "c_flags.WillFlag"]
    Option<WillTopic>,
    #[flag = "c_flags.WillFlag"]
    Option<WillPayload>,
    #[flag = "c_flags.UserNameFlag"]
    Option<Username>,
    #[flag = "c_flags.PasswordFlag"]
    Option<Password>*/
);

#[derive(Clone, Debug, PartialEq)]
pub enum Protocol
{
    MQTT,
    MQIsdp
}

#[async_trait]
impl<R> FromBitReader<R> for Protocol where Self : Sized, R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader(reader : &mut bitreader_async::BitReader<R>) -> Result<Protocol>
    {
        let protocol = UTF8EncodedString::from_bitreader(reader).await?;

        match protocol.as_str()
        {
            "MQTT" => Ok(Protocol::MQTT),
            "MQIsdp" => Ok(Protocol::MQIsdp),
            _ => panic!("Couldn't parse control packet because the client tried to use a protocol other than MQTT!")
        }
    }
}

pub type ProtocolLevel = u8;
pub type KeepAlive = u16;

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
    Reserve : bool
}

pub type ClientID = UTF8EncodedString;
pub type WillProperties = Properties;
pub type WillTopic = UTF8EncodedString;
pub type WillPayload = BinaryData;
pub type Username = UTF8EncodedString;
pub type Password = UTF8EncodedString;

#[cfg(test)]
mod test
{
    use bitreader_async::BitReader;
    use crate::Packet;
    use crate::data_representation::FromBitReader;
    use async_std::io::BufReader;
    use async_std::task;

    #[test]
    fn read_connect()
    {

        task::block_on(async {
            let bytes = [
            0b00010000,//Packet type and ReservedFlags
            0b00010000,
            0b00000000,0b00000100, //Protocol Length
            0b01001101,//M
            0b01010001,//Q
            0b01010100,//T
            0b01010100,//T
            0b00000101,//Protocol Version 5
            0b11001110,//Connect Flags
            0b00000000,0b00001010,//Keep Alive
            0b00000101,//Properties Length
            0b00010001,//Session Expiry Interval identifier
            0b00000000, //Session Expiry Interval
            0b00000000,
            0b00000000,
            0b00001010];

            let underlying_reader : BufReader<&[u8]> = BufReader::new(&bytes);

            let mut reader = BitReader::<BufReader<&[u8]>>::new(underlying_reader);

            let packet = Packet::from_bitreader(&mut reader).await.unwrap();

            print!("Packect : {:#?}", packet);
        });

    }
}