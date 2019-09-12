pub mod protocol;
pub mod connect_flags;
pub mod keep_alive;

use super::Packet;
use super::low_level_read::*;
use futures::io::{BufReader};

use connect_flags::ConnectFlags;

async fn read_connect_packet(mut reader: &mut BufReader<runtime::net::tcp::TcpStream>) -> Packet
{
    let protocol_string : String = read_utf8_encoded_string(&mut reader).await;
    let protocol_version : u8 = read_byte(&mut reader).await;

    let proto = protocol::Protocol::from_string(protocolString.as_str());

    let connect_flags = connect_flags::from_u8(read_byte(&mut reader).await).unwrap();

    let keep_alive_time = read_byte(&mut reader).await as keep_alive::KeepAlive;

    return Packet::CONNECT(proto, protocolVersion as protocol::ProtocolLevel, connect_flags,keep_alive_time);
}