mod fixed_header;
mod properties;
mod reason_code;
mod low_level_read;
mod payload;
mod qos;


//mod connect;
//use connect::{protocol::Protocol, protocol::ProtocolLevel, connect_flags::ConnectFlags, keep_alive::KeepAlive};


use futures::io::{BufReader};
use futures::prelude::*;

use payload::Payload;

//TODO implement this as a trait of a BufferedStream when Rust 
pub async fn read_packet(mut reader: &mut BufReader<runtime::net::tcp::TcpStream>)// -> Packet
{
    //read the fixed header of the control packet
    let header = fixed_header::read_control_packet_type_and_flags(&mut reader).await;

    let remaining_length = low_level_read::read_variable_byte_integer(&mut reader).await;

	let mut remaining_packet_data : Vec<u8> = Vec::with_capacity(remaining_length as usize);
    let read_result = reader.read_exact(&mut remaining_packet_data).await;
	
	/*
	match read_result
	{
		//make sure we read all the bytes we needed
		Ok(n) => assert_eq!(n, remaining_length),
		Err(e) => panic!(e)
	}*/

	println!("header: {:?}", header);

}


#[derive(Debug, Clone, PartialEq)]
pub enum Packet {
	//CONNECT(Protocol, ProtocolLevel, ConnectFlags, KeepAlive),
	CONNACK,
	PUBLISH(Payload),
	PUBACK,
	PUBREC,
	PUBREL,
	PUBCOMP,
	SUBSCRIBE(Payload),
	SUBACK(Payload),
	UNSUBSCRIBE(Payload),
	UNSUBACK(Payload),
	PINGREQ,
	PINGRESP,
	DISCONNECT,
	AUTH
}
