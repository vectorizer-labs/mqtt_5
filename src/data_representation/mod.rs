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
//pub mod reason_code;
pub mod reserved_flags;


use packattack::*;
use super::error::*;

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

#[async_trait]
impl<R> FromReader<MQTTParserError, R> for VariableByteInteger where Self : Sized, R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_reader(reader : &mut R) -> Result<VariableByteInteger>
    {
        let mut multiplier: u32 = 1;
        let mut value: u32 = 0;
        let mut count : u8 = 0;

        let mut encoded_byte : [u8; 1] = [0];

        loop
        {
            count += 1;
            //read the next byte
            reader.read_exact(&mut encoded_byte).await?;

            value += (encoded_byte[0] as u32 & 127) * multiplier;

            //if we exceed the 4 byte limit throw MalformedVariableIntegerError
            if multiplier > 128*128*128 {  return Err(MQTTParserError::MalformedVariableIntegerError); }

            multiplier *= 128;

            if(encoded_byte[0] & 128) == 0 { break; }
        }

        Ok(VariableByteInteger(value))
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

