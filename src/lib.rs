//This file implements the MQTT control packet format as written in the 
//MQTT 5.0 spec here: https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901019
#![feature(arbitrary_enum_discriminant)]
#[macro_use] extern crate failure_derive;

#[macro_use] extern crate packattack_derive;

mod connect;
pub mod data_representation;
pub mod error;

use error::Result;
use data_representation::{ FromBitReader, reserved_flags::ReservedFlags};
use async_std::io::Read;

type RemainingLength = data_representation::VariableByteInteger;

#[derive(Clone, Debug, PartialEq, FromBitReader)]
#[size_in_bits = 4]
#[repr(u8)]
pub enum Packet 
{
    
    CONNECT(connect::Connect) = 0,
    CONNACK(ReservedFlags, RemainingLength) = 1,
    PUBLISH(ReservedFlags, RemainingLength) = 2,
    PUBACK(ReservedFlags, RemainingLength) = 3,
    PUBREC(ReservedFlags, RemainingLength) = 4,
    PUBREL(ReservedFlags, RemainingLength) = 5,
    PUBCOMP(ReservedFlags, RemainingLength) = 6,
    SUBSCRIBE(ReservedFlags, RemainingLength) = 7,
    SUBACK(ReservedFlags, RemainingLength) = 8,
    UNSUBSCRIBE(ReservedFlags, RemainingLength) = 9,
    UNSUBACK(ReservedFlags, RemainingLength) = 10,
    PINGREQ(ReservedFlags, RemainingLength) = 11,
    PINGRESP(ReservedFlags, RemainingLength) = 12,
    DISCONNECT(ReservedFlags, RemainingLength) = 13,
    AUTH(ReservedFlags, RemainingLength) = 14
}
