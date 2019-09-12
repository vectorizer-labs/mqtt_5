#[derive(Debug, Clone, PartialEq)]
pub enum Protocol
{
    MQTT,
    MQIsdp
}

impl Protocol
{
    pub fn from_string(protocol : &str) -> Protocol
    {
        match protocol 
        {
            "MQTT" => Protocol::MQTT,
            _ => panic!("Couldn't parse control packet because the client tried to use a protocol other than MQTT!")
        }
    }
}

pub type ProtocolLevel = u8;