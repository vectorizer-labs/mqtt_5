use super::{qos::QoS};
use packattack::*;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, FromBitReader)]
pub struct ReservedFlags
{
    pub dup : bool,
    pub qos : QoS,
    pub retain : bool,
}