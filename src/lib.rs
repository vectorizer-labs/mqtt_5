#![feature(arbitrary_enum_discriminant)]
#![feature(box_syntax, test, fmt_internals)]

#[macro_use]
extern crate packattack_derive;

use packattack::FromBytes;
use packattack::FromByte;


mod connect;
mod qos;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, FromBytes)]
#[repr(u8)]
enum Packet 
{
    CONNECT(connect::Protocol, connect::ProtocolLevel, connect::ConnectFlags, connect::KeepAlive) = 0,
    CONNACK = 1,
    PUBLISH = 2,
    PUBACK = 3,
    PUBREC = 4,
    PUBREL = 5,
    PUBCOMP = 6,
    SUBSCRIBE = 7,
    SUBACK = 8,
    UNSUBSCRIBE = 9,
    UNSUBACK = 10,
    PINGREQ = 11,
    PINGRESP = 12,
    DISCONNECT = 13,
    AUTH = 14
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn build_packet_from_bytes()
    {
        let bytes : [u8;3] = [8,0,0];

        let mut count = 0;

        let parsed_packet = Packet::read_from_bytes(&bytes, &mut count);

        assert_eq!(Packet::SUBACK, parsed_packet);
    }

    #[test]
    fn build_Connect_from_byte()
    {
        let byte : [u8;1] = [0b01110101];

        let mut count = 0;

        let parsed_connect = connect::ConnectFlags::read_from_bytes(&byte, &mut count);

        println!("connect: {:#? } ", parsed_connect);

        //assert_eq!(Packet::SUBACK, parsed_packet);
    }
}