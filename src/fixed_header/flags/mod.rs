const DUP_BIT : u8 = 3;
const RETAIN_BIT : u8 = 0;
const BLANK_FLAG : u8 = 0b0000u8;
const BIT1_FLAG : u8 = 0b0010u8;

pub mod duplicate;
pub mod retain;

use super::FixedHeader;
use duplicate::Duplicate;
use super::super::qos::QoS;
use retain::Retain;

use std::error;
use std::fmt;

//traits for reading the publish flags
pub trait Flags
{
    fn into_DUP_QOS_RETAIN(self) -> FixedHeader;
    fn match_packet_type(self, header : &FixedHeader) -> Result<(), IncorrectFlagError>;
}

impl Flags for u8
{
    fn into_DUP_QOS_RETAIN(self) -> FixedHeader
    {
        let raw_qos = self & 0b0110u8; 

        FixedHeader::PUBLISH(
            Duplicate::from_bool(get_bit_at(self, DUP_BIT)),
            QoS::from_u8(raw_qos),
            Retain:: from_bool(get_bit_at(self, RETAIN_BIT))
        )

    } 

    fn match_packet_type(self, header : &FixedHeader) -> Result<(), IncorrectFlagError>
    {
        match header
        {
            //If its a publish packet we can't really know if its malformed based on the flag
            //FixedHeader::PUBLISH => {},
            FixedHeader::PUBREL | FixedHeader::SUBSCRIBE | FixedHeader::UNSUBSCRIBE => 
            { 
                //These packets should have bit 1 enabled. If they don't they are malformed.
                match self
                {
                    BIT1_FLAG => Ok(()),
                    _ => Err(IncorrectFlagError::new(BIT1_FLAG,self))
                } 
            },
            _ => 
            { 
                //All other packets should have blank flags. If they don't they are malformed.
                match self
                {
                    BLANK_FLAG => Ok(()),
                    _ => Err(IncorrectFlagError::new(BLANK_FLAG,self))
                }
            }
        }
    }
}

///adapted from: https://www.reddit.com/r/rust/comments/3xgeo0/biginner_question_how_can_i_get_the_value_of_a/
/// gets the bit at position `n`
/// MQTT: Bits in a byte are labelled 7 to 0. Bit number 7 is the most significant bit, the least significant bit is assigned bit number 0.
pub fn get_bit_at(input: u8, n: u8) -> bool 
{
    if n < 8 { input & (1 << n) != 0 } 
    else { panic!("Tried to read bit outside of range!"); }
}

//type Result<T> = std::result::Result<T, FlagError>;

//Flag error type
#[derive(Debug, Clone)]
pub struct IncorrectFlagError
{
    expected_value : u8,
    found_value : u8
}

impl fmt::Display for IncorrectFlagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Packet flags did not match packet header! Expected Value: {} found : {}", self.expected_value,self.found_value)
    }
}

// This is important for other errors to wrap this one.
impl error::Error for IncorrectFlagError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl IncorrectFlagError {
    fn new(expected_value : u8, found_value : u8 ) -> IncorrectFlagError 
    {
        IncorrectFlagError{ expected_value : expected_value, found_value : found_value }
    }
}