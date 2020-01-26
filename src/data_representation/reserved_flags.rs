use super::qos::QoS;
use packattack::*;

#[derive(Clone, Debug, PartialEq, FromBytes)]
pub struct ReservedFlags
{
    pub dup : bool,
    pub qos : QoS,
    pub retain : bool,
}