pub type Byte = u8;
pub type TwoByteInteger = u16;
pub type FourByteInteger = u32;
pub type UTF8EncodedString = String;
pub type BinaryData = Vec<u8>;

pub struct VariableByteInteger(u32);


pub mod properties;
pub mod qos;
pub mod reason_code;


use bitreader::BitReader;
use std::result;
use std::error::Error;
use std::error;
use std::fmt;

//TODO: Expand this definition to more errors
/// Result type for those BitReader operations that can fail.
pub type Result<T> = result::Result<T, Box<dyn Error>>;

pub trait FromBitReader: Sized
{
    fn from_bitreader(reader : &mut BitReader) -> Result<Self>;
}

impl FromBitReader for bool
{
    fn from_bitreader(reader : &mut BitReader) -> Result<bool>
    {
        Ok(reader.read_bool()?)
    }
}

impl FromBitReader for Byte
{
    fn from_bitreader(reader : &mut BitReader) -> Result<Byte>
    {
        Ok(reader.read_u8(8)?)
    }
}

impl FromBitReader for TwoByteInteger
{
    fn from_bitreader(reader : &mut BitReader) -> Result<TwoByteInteger>
    {
        Ok(reader.read_u16(16)?)
    }
}

impl FromBitReader for FourByteInteger
{
    fn from_bitreader(reader : &mut BitReader) -> Result<FourByteInteger>
    {
        Ok(reader.read_u32(32)?)
    }
}

impl FromBitReader for UTF8EncodedString
{
    fn from_bitreader(reader : &mut BitReader) -> Result<UTF8EncodedString>
    {
        let length = reader.read_u16(16)?;

        let mut vec_buffer : Vec<u8> = Vec::with_capacity(length as usize);

        reader.read_u8_slice(vec_buffer.as_mut_slice())?;

        let result = String::from_utf8(vec_buffer)?;

        Ok(result)
    }
}

impl FromBitReader for BinaryData
{
    fn from_bitreader(reader : &mut BitReader) -> Result<BinaryData>
    {
        let length = reader.read_u16(16)?;

        let mut vec : Vec<u8> = Vec::with_capacity(length as usize);

        reader.read_u8_slice(vec.as_mut_slice())?;

        Ok(vec)
    }
}

impl FromBitReader for VariableByteInteger
{
    fn from_bitreader(reader : &mut BitReader) -> Result<VariableByteInteger>
    {
        let mut multiplier: u32 = 1;
        let mut value: u32 = 0;
        
        let mut raw_byte : [u8;1] = [0];
        while (raw_byte[0] & 128)  != 0
        {
            //read the next byte
            reader.read_u8_slice(&mut raw_byte)?;

            let encoded_byte: u32 = (raw_byte[0] as u32 & 127) * multiplier;
            value += encoded_byte;

            //if we exceed the 4 byte limit throw MalformedVariableIntegerError
            if multiplier > 128*128*128 { 
                return Err(Box::new(MalformedVariableIntegerError))
            }

            multiplier *= 128;
        }

        Ok( VariableByteInteger(value))
    }
}

impl From<VariableByteInteger> for usize 
{
    fn from(v: VariableByteInteger ) -> usize {
        v.0 as usize
    }
}

pub struct MalformedVariableIntegerError;

impl fmt::Debug for MalformedVariableIntegerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse VariableByteInteger!")
    }
}

impl fmt::Display for MalformedVariableIntegerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse VariableByteInteger!")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for MalformedVariableIntegerError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}