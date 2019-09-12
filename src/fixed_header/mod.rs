use futures::io::{BufReader};
use super::low_level_read::*;
use std::fmt::{self, Debug, Display};

mod flags;
use flags::Flags;
use flags::duplicate::Duplicate;
use super::qos::QoS;
use flags::retain::Retain;


//This file implements the MQTT control packet format as written in the 
//MQTT 5.0 spec here: https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901019


pub enum FixedHeader
{
    RESERVED,
    CONNECT,
    CONNACK,
    PUBLISH(Duplicate, QoS, Retain),
    PUBACK,
    PUBREC,
    PUBREL,
    PUBCOMP,
    SUBSCRIBE,
    SUBACK,
    UNSUBSCRIBE,
    UNSUBACK,
    PINGREQ,
    PINGRESP,
    DISCONNECT,
    AUTH
}

impl FixedHeader
{
    fn from_u8_and_flags(pType : u8, f : u8) -> FixedHeader
    {
        let header = match pType
        {
            0 =>  FixedHeader::RESERVED,
            1 =>  FixedHeader::CONNECT,
            2 =>  FixedHeader::CONNACK,
            3 =>  f.into_DUP_QOS_RETAIN(),
            4 =>  FixedHeader::PUBACK,
            5 =>  FixedHeader::PUBREC,
            6 =>  FixedHeader::PUBREL,
            7 =>  FixedHeader::PUBCOMP,
            8 =>  FixedHeader::SUBSCRIBE,
            9 =>  FixedHeader::SUBACK,
            10 => FixedHeader::UNSUBSCRIBE,
            11 => FixedHeader::UNSUBACK,
            12 => FixedHeader::PINGREQ,
            13 => FixedHeader::PINGRESP,
            14 => FixedHeader::DISCONNECT,
            15 => FixedHeader::AUTH,
            _ => panic!("couldn't parse control packet type because it was malformed")
        };

        return match f.match_packet_type(&header)
        {
            Ok(h) => header,
            Err(e) => panic!(e)
        };

    }
}

pub async fn read_control_packet_type_and_flags(mut reader: &mut BufReader<runtime::net::tcp::TcpStream>) -> FixedHeader
{
    let first_byte = read_byte(&mut reader).await;

    //get the four bit values via bitwise AND
    let raw_type : u8 = first_byte & 0b00001111u8;
    let raw_flag : u8 = first_byte & 0b1111u8;

    FixedHeader::from_u8_and_flags(raw_type, raw_flag)

}

impl std::fmt::Debug for FixedHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}