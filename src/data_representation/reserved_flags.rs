use super::super::error::Result;
use super::{ FromBitReader, qos::QoS};
use async_std::io::Read;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, FromBitReader)]
pub struct ReservedFlags
{
    dup : bool,
    qos : QoS,
    retain : bool,
}