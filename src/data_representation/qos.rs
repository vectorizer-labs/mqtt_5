use packattack::*;

#[derive(Clone, Copy, Debug, PartialEq, FromBytes)]
#[size_in_bits = 2]
#[from_bits]
#[repr(u8)]
pub enum QoS
{
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2
}