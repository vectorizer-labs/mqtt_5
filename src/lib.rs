#![feature(arbitrary_enum_discriminant)]
#![feature(test)]

#[macro_use] extern crate failure;

mod connect;
/*mod connack;
//mod publish; use publish::{ PacketIdentifier, TopicName, Payload };
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
mod auth;*/

pub mod data_representation;
pub mod error;

pub type ERROR = error::MQTTParserError;

use packattack::*;

use data_representation::{ reserved_flags::ReservedFlags, RemainingLength, properties::Properties, qos::QoS };


//This repo implements the MQTT control packet format as written in the 
//MQTT 5.0 spec here: https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901019
#[derive(Clone, Debug, PartialEq, FromReader)]
#[from_bits]
#[size_in_bits = 4]
#[repr(u8)]
pub enum Packet 
{
    CONNECT(#[from_bits] ReservedFlags, connect::Connect) = 1,
    //CONNACK(#[from_bits] ReservedFlags, connack::Connack) = 2,
    //PUBLISH(#[expose = "r_flags"] #[from_bits] ReservedFlags, publish::Publish) = 3,
    //PUBACK(#[from_bits] ReservedFlags, puback::Puback) = 4,
    //PUBREC(#[from_bits] ReservedFlags, pubrec::Pubrec) = 5,
    //PUBREL(#[from_bits] ReservedFlags, pubrel::Pubrel) = 6,
    //PUBCOMP(#[from_bits] ReservedFlags, pubcomp::Pubcomp) = 7,
    //SUBSCRIBE(#[from_bits] ReservedFlags, subscribe::Subscribe) = 8,
    //SUBACK(()) = 9,
    //UNSUBSCRIBE(()) = 10,
    //UNSUBACK(()) = 11,
    //PINGREQ(()) = 12,
    //PINGRESP(()) = 13,
    //DISCONNECT(()) = 14,
    //AUTH(()) = 15
}


//TODO: copy the first byte from enum into from_bytes of first type
//TODO: fix from_bytes for derived types
#[cfg(test)]
mod test
{
    use crate::Packet;
    use super::*;
    use async_std::task;

    extern crate test;
    use test::Bencher;
    
    #[bench]
    fn bench_read_connect(b: &mut Bencher)
    {   
        b.iter(||{

            task::block_on(async {

                let bytes = [
                0b00010000,//Packet type and ReservedFlags
                0b00011001,
                0b00000000,0b00000100, //Protocol Length
                0b01001101,//M
                0b01010001,//Q
                0b01010100,//T
                0b01010100,//T
                0b00000101,//Protocol Version 5
                0b00001010,//Connect Flags
                0b00000000,0b00001010,//Keep Alive
                0b00000101,//Properties Length
                0b00010001,//Session Expiry Interval identifier
                0b00000000, //Session Expiry Interval
                0b00000000,
                0b00000000,
                0b00001010,
                0b00000000,//ClientID len
                0b00000111,
                0b01001001,//IAMCOOL Start
                0b01000001,
                0b01001101,
                0b01000011,
                0b01001111,
                0b01001111,
                0b01101100];

                let packet = Packet::from_reader(&mut bytes.as_ref()).await.unwrap();

                print!("Packect : {:#?}", packet);

                packet

            });
        });

    }
}