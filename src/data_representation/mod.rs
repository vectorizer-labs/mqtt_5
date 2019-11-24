pub type Byte = u8;
pub type TwoByteInteger = u16;
pub type FourByteInteger = u32;
pub type UTF8EncodedString = String;
pub type BinaryData = Vec<u8>;


pub mod properties;
pub mod qos;
pub mod reason_code;


use bitreader::BitReader;

//TODO: Expand this definition to more errors
/// Result type for those BitReader operations that can fail.
pub type Result<T> = result::Result<T, bitreader::BitReaderError>;

use std::result;

pub trait FromBitReader: Sized
{
    fn from_bitreader(reader : &mut BitReader) -> Result<Self>;
}

impl FromBitReader for bool
{
    fn from_bitreader(reader : &mut BitReader) -> Result<bool>
    {
        reader.read_bool()
    }
}

impl FromBitReader for Byte
{
    fn from_bitreader(reader : &mut BitReader) -> Result<Byte>
    {
        reader.read_u8(8)
    }
}

impl FromBitReader for TwoByteInteger
{
    fn from_bitreader(reader : &mut BitReader) -> Result<TwoByteInteger>
    {
        reader.read_u16(16)
    }
}

impl FromBitReader for FourByteInteger
{
    fn from_bitreader(reader : &mut BitReader) -> Result<FourByteInteger>
    {
        reader.read_u32(32)
    }
}

impl FromBitReader for UTF8EncodedString
{
    fn from_bitreader(reader : &mut BitReader) -> Result<UTF8EncodedString>
    {
        let length = reader.read_u16(16).unwrap();

        let mut vec_buffer : Vec<u8> = Vec::with_capacity(length as usize);

        reader.read_u8_slice(vec_buffer.as_mut_slice()).unwrap();

        let result = String::from_utf8(vec_buffer).unwrap();

        Ok(result)
    }
}

impl FromBitReader for BinaryData
{
    fn from_bitreader(reader : &mut BitReader) -> Result<BinaryData>
    {
        let length = reader.read_u16(16).unwrap();

        let mut vec : Vec<u8> = Vec::with_capacity(length as usize);

        reader.read_u8_slice(vec.as_mut_slice()).unwrap();

        Ok(vec)
    }
}