pub type Byte = u8;
pub type TwoByteInteger = u16;
pub type FourByteInteger = u32;
pub type UTF8EncodedString = String;
pub type BinaryData = Vec<u8>;

pub type RemainingLength = VariableByteInteger;

#[derive(Clone, Debug, PartialEq)]
pub struct VariableByteInteger(u32,u8);

pub mod properties;
pub mod qos;
pub mod reason_code;
pub mod reserved_flags;

use packattack::*;
use super::error::*;

#[async_trait]
impl<R> FromBitReader<MQTTParserError, R> for UTF8EncodedString where Self : Sized, R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader(reader : &mut BitReader<R>) -> Result<UTF8EncodedString>
    {
        let length = reader.read_aligned_be::<u16>().await?;

        let vec_buffer : Vec<u8> = reader.read_u8_slice_aligned(length as usize).await?;

        let result = String::from_utf8(vec_buffer)?;

        Ok(result)
    }
}

#[async_trait]
impl<R> FromBitReader<MQTTParserError, R> for BinaryData where Self : Sized, R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader(reader : &mut BitReader<R>) -> Result<BinaryData>
    {
        let len = reader.read_aligned_be::<u16>().await?;

        Ok(reader.read_u8_slice_aligned(len as usize).await?)
    }
}

#[async_trait]
impl<R> FromBitReader<MQTTParserError, R> for VariableByteInteger where Self : Sized, R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader(reader : &mut BitReader<R>) -> Result<VariableByteInteger>
    {
        let mut multiplier: u32 = 1;
        let mut value: u32 = 0;
        let mut count : u8 = 0;

        loop
        {
            count += 1;
            //read the next byte
            let encoded_byte = reader.read_aligned_be::<u8>().await?;

            value += (encoded_byte as u32 & 127) * multiplier;

            //if we exceed the 4 byte limit throw MalformedVariableIntegerError
            if multiplier > 128*128*128 {  return Err(MQTTParserError::MalformedVariableIntegerError); }

            multiplier *= 128;

            if(encoded_byte & 128) == 0 { break; }
        }

        Ok(VariableByteInteger(value,count))
    }
}

impl From<&VariableByteInteger> for usize 
{
    fn from(v: &VariableByteInteger ) -> usize 
    {
        v.0.clone() as usize
    }
}

impl From<VariableByteInteger> for usize 
{
    fn from(v: VariableByteInteger ) -> usize 
    {
        v.0 as usize
    }
}

impl VariableByteInteger
{
    pub fn size(&self) -> usize
    {
        self.1.clone() as usize
    }
}

/*
#[async_trait]
impl<R, T> FromBitReaderWithLength<MQTTParserError, R> for Vec<T> 
where T : Sized + FromBitReader<MQTTParserError, R> + std::marker::Send, 
      R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader_with_length(reader : &mut BitReader<R>, len : usize) -> Result<Vec<T>>
    {
        let mut items : Vec<T> = Vec::new();

        let end = reader.byte_count() + len;

        while reader.byte_count() < end 
        {
            items.push(<T>::from_bitreader(reader).await?);
        }

        Ok(items)
    }
}*/

