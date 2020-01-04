use super::super::error::Result;
use super::{ FromBitReader, qos::QoS};
use async_std::io::Read;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, FromBitReader)]
pub struct ReservedFlags
{
    pub dup : bool,
    pub qos : QoS,
    pub retain : bool,
}