use super::FromBitReader;
use async_std::io::Read;
use super::super::error::Result;

#[derive(Clone, Copy, Debug, PartialEq, FromBitReader)]
#[size_in_bits = 2]
#[repr(u8)]
pub enum QoS
{
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2
}
