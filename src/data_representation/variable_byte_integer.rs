use super::*;

impl Bitsize for VariableByteInteger 
{
    const SIZE_IN_BITS : usize = 0; 
}

#[async_trait]
impl<R> FromReader<MQTTParserError, R> for VariableByteInteger where Self : Sized, R : Read + std::marker::Unpin + std::marker::Send
{
    async fn from_reader(reader : &mut R) -> Result<VariableByteInteger>
    {
        let mut multiplier: u32 = 1;
        let mut value: u32 = 0;
        //let mut count : u8 = 0;

        let mut encoded_byte : [u8; 1] = [0];

        loop
        {
            //count += 1;
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

impl FromBytes<MQTTParserError, &mut &[u8]> for VariableByteInteger where Self : Sized
{
    #[inline]
    fn from_bytes(bytes : &mut &[u8]) -> Result<VariableByteInteger>
    {
        let mut multiplier: u32 = 1;
        let mut value: u32 = 0;

        loop
        {
            //TODO: assert len > 1
            //TODO: make a custom slice reader trait
            //read a byte from the slice
            let (encoded_byte, b) = bytes.split_at(1);
            //update slice len
            *bytes = b;

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