use packattack::FromByte;

#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug, PartialEq, FromByte)]
#[size_in_bits = 2]
#[repr(u8)]
pub enum QoS
{
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2
}
