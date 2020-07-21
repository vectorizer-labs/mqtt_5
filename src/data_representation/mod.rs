pub type Byte = u8;
pub type TwoByteInteger = u16;
pub type FourByteInteger = u32;
pub type UTF8EncodedString = String;
pub type BinaryData = Vec<u8>;

pub type RemainingLength = VariableByteInteger;

#[derive(Clone, Debug, PartialEq)]
pub struct VariableByteInteger(u32);

pub mod properties;
pub mod qos;
pub mod reason_code;
pub mod reserved_flags;
pub mod variable_byte_integer;

use packattack::*;
use super::error::*;
use variable_byte_integer::*;

//#[length = Expr] should resolve to a (size : u8, value : Vec<u8>)
//TODO: Fix integers and fix from_bytes holes in general
#[async_trait]
impl<R> FromReader<MQTTParserError, R> for UTF8EncodedString where Self : Sized, 
R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_reader(reader : &mut R) -> Result<UTF8EncodedString>
    {
        let mut length_array : [u8;2] = [0;2];
        reader.read_exact(&mut length_array).await?;

        let length : u16 = <u16>::from_be_bytes(length_array);

        //have to assign zeroes to the first count # of items
        //other wise it just thinks the slice is size 0
        let mut vec_buffer : Vec<u8> = vec![0; length as usize];

        reader.read_exact(vec_buffer.as_mut_slice()).await?;

        let result = String::from_utf8(vec_buffer)?;

        Ok(result)
    }
}

impl FromBytes<MQTTParserError, &mut &[u8]> for UTF8EncodedString where Self : Sized
{
    #[inline]
    fn from_bytes(bytes : &mut &[u8]) -> Result<UTF8EncodedString>
    {
        //TODO: assert len > 2
        let (length_value, b) = bytes.split_at(std::mem::size_of::<u16>());
        //update slice len
        *bytes = b;

        let length : u16 = <u16>::from_be_bytes(length_value.try_into()?);

        let (string_value, b) = bytes.split_at(length as usize);
        *bytes = b;

        let result = String::from_utf8(string_value.to_vec())?;

        Ok(result)
    }
}

#[async_trait]
impl<R> FromReader<MQTTParserError, R> for BinaryData where Self : Sized, R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_reader(reader : &mut R) -> Result<BinaryData>
    {
        let mut length_array : [u8;2] = [0;2];
        reader.read_exact(&mut length_array).await?;

        let length : u16 = <u16>::from_be_bytes(length_array);

        //have to assign zeroes to the first count # of items
        //other wise it just thinks the slice is size 0
        let mut vec_buffer : Vec<u8> = vec![0; length as usize];

        reader.read_exact(vec_buffer.as_mut_slice()).await?;

        Ok(vec_buffer)
    }
}