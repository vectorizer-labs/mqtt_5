#![feature(arbitrary_enum_discriminant)]
#![feature(box_syntax, test, fmt_internals)]

#[macro_use]
extern crate packattack_derive;

mod connect;
mod data_representation;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
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