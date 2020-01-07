//This file implements the MQTT control packet format as written in the 
//MQTT 5.0 spec here: https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901019
#![feature(arbitrary_enum_discriminant)]
#[macro_use] extern crate failure;

#[macro_use] extern crate packattack_derive;

mod connect;
mod connack;
mod publish;
mod puback;
mod pubrec;
mod pubrel;
mod pubcomp;
mod subscribe;
mod suback;
mod unsubscribe;
mod unsuback;
mod pingreq;
mod pingresp;
mod disconnect;
mod auth;

pub mod data_representation;
pub mod error;

pub type ERROR = error::MQTTParserError;

use packattack::*;

#[derive(Clone, Debug, PartialEq, FromBitReader)]
#[size_in_bits = 4]
#[repr(u8)]
pub enum Packet 
{
    CONNECT(connect::Connect) = 1,
    CONNACK(connack::Connack) = 2,
    PUBLISH(publish::Publish) = 3,
    PUBACK(puback::Puback) = 4,
    PUBREC(pubrec::Pubrec) = 5,
    PUBREL(pubrel::Pubrel) = 6,
    PUBCOMP(pubcomp::Pubcomp) = 7,
    SUBSCRIBE(()) = 8,
    SUBACK(()) = 9,
    UNSUBSCRIBE(()) = 10,
    UNSUBACK(()) = 11,
    PINGREQ(()) = 12,
    PINGRESP(()) = 13,
    DISCONNECT(()) = 14,
    AUTH(()) = 15
}
