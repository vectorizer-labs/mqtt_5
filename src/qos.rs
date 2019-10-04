use packattack::FromBytes;

#[derive(Clone, Copy, Debug, PartialEq, FromBytes)]
#[repr(u8)]
pub enum QoS
{
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2
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
