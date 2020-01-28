
use super::data_representation::{
    properties::Properties, 
    qos::QoS,
    UTF8EncodedString, 
    RemainingLength,
    BinaryData
};

use packattack::*;
 
#[derive(Clone, Debug, PartialEq, FromReader)]
#[hint = "RemainingLength"]
pub struct Connect
(
    Protocol,
    #[from_bytes]
    ProtocolLevel,
    #[expose = "c_flags"]
    #[from_bytes]
    ConnectFlags,
    #[from_bytes]
    KeepAlive,
    Properties,
    ClientID,
    #[flag = "c_flags.WillFlag"]
    Option<WillProperties>,
    #[flag = "c_flags.WillFlag"]
    Option<WillTopic>,
    #[flag = "c_flags.WillFlag"]
    Option<WillPayload>,
    #[flag = "c_flags.UserNameFlag"]
    Option<Username>,
    #[flag = "c_flags.PasswordFlag"]
    Option<Password>
);


#[derive(Clone, Debug, PartialEq, FromReader)]
#[size_in_bits = "UTF8EncodedString"]
#[str]
pub enum Protocol
{
    #[discriminant = "MQTT"]
    MQTT,
    #[discriminant = "MQIsdp"]
    MQIsdp//Older name for MQTT
}

// _ => panic!("Couldn't parse control packet because the client tried to use a protocol other than MQTT!")

pub type ProtocolLevel = u8;
pub type KeepAlive = u16;

#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug, PartialEq, FromBytes)]
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


