//This file implements the MQTT control packet format as written in the 
//MQTT 5.0 spec here: https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901019
#![feature(arbitrary_enum_discriminant)]
#[macro_use] extern crate failure_derive;

#[macro_use] extern crate packattack_derive;

mod connect;
mod connack;
mod publish;


pub mod data_representation;
pub mod error;

use error::Result;
use data_representation::{ FromBitReader};
use async_std::io::Read;



#[derive(Clone, Debug, PartialEq, FromBitReader)]
#[size_in_bits = 4]
#[repr(u8)]
pub enum Packet 
{
    
    CONNECT(connect::Connect) = 1,
    CONNACK(connack::Connack) = 2,
    PUBLISH(()) = 3,
    PUBACK(()) = 4,
    PUBREC(()) = 5,
    PUBREL(()) = 6,
    PUBCOMP(()) = 7,
    SUBSCRIBE(()) = 8,
    SUBACK(()) = 9,
    UNSUBSCRIBE(()) = 10,
    UNSUBACK(()) = 11,
    PINGREQ(()) = 12,
    PINGRESP(()) = 13,
    DISCONNECT(()) = 14,
    AUTH(()) = 15
}
