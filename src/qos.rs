#[derive(Debug, Clone, PartialEq)]
pub enum QoS
{
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce
}

impl QoS
{
    pub fn from_u8(code : u8) -> QoS
    {
        match code
        {
            0 => QoS::AtMostOnce,
            1 => QoS::AtLeastOnce,
            2 => QoS::ExactlyOnce,
            _ => panic!("Couldn't parse control packet QoS because it was malformed")
        }
    }
}
