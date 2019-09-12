use futures::prelude::*;
use futures::io::{BufReader};
use std::convert::TryInto;

//This file implements data reading traits corresponding to the Data Representation section of the 
//MQTT 5.0 spec here: https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901006

//TODO: Navigate the dangerous waters of asynchronously reading from an Async
//BufferedReader without copying into a local variable
//I.E. Implement these traits using zero-copy 
//basically we'll have to manage buffer flushing by hand
//but it should give us a noticable decrease in memory usage

//Alternatively it could be better to allocate local variables 
//if we could build different parts of our packet asynchronously
//however we should still be able to implement this without allocating local variables
//it just sounds like a lifetime definition nightmare
//I'm not sure how lifetimes interact with the buffer and according to the github page 
//multiple async lifetimes on a single task may not be implemented yet. 
/*pub trait MQTTPacketStream
{
    fn read_byte(&mut self) -> std::io::Result<u8>;
    fn read_two_byte_integer(&mut self) -> std::io::Result<u16>;
    fn read_four_byte_integer(&mut self) -> std::io::Result<u32>;
    fn read_utf8_encoded_string(&mut self) -> String;
}*/

//TODO: Theres currently no support for async trait functions because presumably they need to 
//be wrapped in ARC to be thread safe (but maybe not in all cases)
//at any rate that functionality isn't built into the compiler right now
//but when it is we'll convert this into a trait

//impl MQTTPacketStream for BufReader<runtime::net::tcp::TcpStream>
//{

    
    pub async fn read_byte(reader: &mut BufReader<runtime::net::tcp::TcpStream>) -> u8
    {
        let mut buffer : [u8;1] = [0];
        let bytecount = reader.read_exact(&mut buffer).await;
        //TODO: convert this to a result to pass it up
        u8::from_be_bytes(buffer)
    }

    pub async fn read_two_byte_integer(reader: &mut BufReader<runtime::net::tcp::TcpStream>) -> u16
    {
        let mut buffer : [u8;2] = [0,0];
        let bytecount = reader.read_exact(&mut buffer).await;
        //TODO: convert this to a result to pass it up
        u16::from_be_bytes(buffer)
    }

    pub async fn read_four_byte_integer(reader: &mut BufReader<runtime::net::tcp::TcpStream>) -> u32
    {
        let mut buffer : [u8;4] = [0,0,0,0];
        let bytecount = reader.read_exact(&mut buffer).await;
        //TODO: convert this to a result to pass it up
        u32::from_be_bytes(buffer)
    }

    pub async fn read_utf8_encoded_string(mut reader: &mut BufReader<runtime::net::tcp::TcpStream>) -> String
    {
        let length : u16 = read_two_byte_integer(&mut reader).await;
        let mut buffer : Vec<u8> = Vec::with_capacity(length as usize);
        let bytecount = reader.read_exact(&mut buffer).await;

        String::from_utf8(buffer).unwrap()
    }


    //MQTT 5.0 Spec: 
    //multiplier = 1
    //value = 0
    //do
    //    encodedByte = 'next byte from stream'
    //    value += (encodedByte AND 127) * multiplier
    //    if (multiplier > 128*128*128)
    //        throw Error(Malformed Variable Byte Integer)
    //    multiplier *= 128
    //while ((encodedByte AND 128) != 0)

    //the largest variable byte size is 4 so the value will be less than max(u32)
    pub async fn read_variable_byte_integer(mut reader: &mut BufReader<runtime::net::tcp::TcpStream>) -> u32
    {
        let mut value: u32 = 0;
        let mut multiplier: u32 = 1;
        loop
        {
            let encoded_byte : u32 = read_byte(&mut reader).await as u32;
            value += (encoded_byte & 127) * multiplier;

            //if we exceed the 4 byte limit panic
            //TODO: pass up a result
            if multiplier > 128*128*128 { panic!("Malformed Variable Byte Integer"); }

            multiplier *= 128;

            if (encoded_byte & 128) == 0 { break; }
        }

        value
    }

    pub async fn read_binary_data(mut reader: &mut BufReader<runtime::net::tcp::TcpStream>) -> Vec<u8>
    {
        let length : u16 = read_two_byte_integer(&mut reader).await;
        let mut buffer : Vec<u8> = Vec::with_capacity(length as usize);
        let bytecount = reader.read_exact(&mut buffer).await;

        buffer
    }

//}