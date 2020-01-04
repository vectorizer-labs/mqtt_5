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

use async_trait::async_trait;
use bitreader_async::BitReader;
use async_std::io::{ Read };
use super::error::Result;

#[async_trait]
pub trait FromBitReader<R> where Self : Sized, R : Read + std::marker::Unpin + std::marker::Send + std::marker::Sync
{
    async fn from_bitreader(reader : &mut BitReader<R>) -> Result<Self>;
}

#[async_trait]
impl<R> FromBitReader<R> for () where Self : Sized, R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader(_reader : &mut BitReader<R>) -> Result<()>
    {
        Ok(())
    }
}

#[async_trait]
impl<R> FromBitReader<R> for bool where Self : Sized, R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader(reader : &mut BitReader<R>) -> Result<bool>
    {
        Ok(reader.read_be_bits(1).await? != 0)
    }
}

#[async_trait]
impl<R> FromBitReader<R> for Byte where Self : Sized, R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader(reader : &mut BitReader<R>) -> Result<Byte>
    {
        Ok(reader.read_aligned_be::<u8>().await?)
    }
}

#[async_trait]
impl<R> FromBitReader<R> for TwoByteInteger where Self : Sized, R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader(reader : &mut BitReader<R>) -> Result<TwoByteInteger>
    {
        Ok(reader.read_aligned_be::<u16>().await?)
    }
}

#[async_trait]
impl<R> FromBitReader<R> for FourByteInteger where Self : Sized, R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader(reader : &mut BitReader<R>) -> Result<FourByteInteger>
    {
        Ok(reader.read_aligned_be::<u32>().await?)
    }
}

#[async_trait]
impl<R> FromBitReader<R> for UTF8EncodedString where Self : Sized, R : Read + std::marker::Unpin + std::marker::Send
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
impl<R> FromBitReader<R> for BinaryData where Self : Sized, R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader(reader : &mut BitReader<R>) -> Result<BinaryData>
    {
        let length = reader.read_aligned_be::<u16>().await?;

        let vec_buffer : Vec<u8> = reader.read_u8_slice_aligned(length as usize).await?;

        Ok(vec_buffer)
    }
}

#[async_trait]
impl<R> FromBitReader<R> for VariableByteInteger where Self : Sized, R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_bitreader(reader : &mut BitReader<R>) -> Result<VariableByteInteger>
    {
        let mut multiplier: u32 = 1;
        let mut value: u32 = 0;

        loop
        {
            //read the next byte
            let encoded_byte = reader.read_aligned_be::<u8>().await?;

            value += (encoded_byte as u32 & 127) * multiplier;

            //if we exceed the 4 byte limit throw MalformedVariableIntegerError
            if multiplier > 128*128*128 {  panic!("bad VariableByteInteger") /*return Err(Box::new(MalformedVariableIntegerError))*/ }

            multiplier *= 128;

            if(encoded_byte & 128) == 0 { break; }
        }

        Ok(VariableByteInteger(value))
    }
}

impl From<VariableByteInteger> for usize 
{
    fn from(v: VariableByteInteger ) -> usize 
    {
        v.0 as usize
    }
}

