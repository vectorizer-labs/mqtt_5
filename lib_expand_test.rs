
impl FromBytes for Packet
{
    # [allow (trivial_numeric_casts)] 
    fn read_from_bytes (bytes : & [u8]) -> Option < & Self >
    {
        match & bytes [0]
        {
            Packet :: CONNACK as u8 => Some (Packet :: CONNACK), 
            Packet :: PUBLISH as u8 => Some (Packet :: PUBLISH), 
            Packet :: PUBACK as u8 => Some (Packet :: PUBACK), 
            Packet :: PUBREC as u8 => Some (Packet :: PUBREC), 
            Packet :: PUBREL as u8 => Some(Packet :: PUBREL), 
            Packet :: PUBCOMP as u8 => Some(Packet :: PUBCOMP), 
            Packet :: SUBSCRIBE as u8 => Some(Packet :: SUBSCRIBE), 
            Packet :: SUBACK as u8 => Some(Packet :: SUBACK(< u8 > :: read_from_bytes (& bytes), < u8 > :: read_from_bytes(& bytes))), 
            Packet :: UNSUBSCRIBE as u8 => Some(Packet :: UNSUBSCRIBE), 
            Packet :: UNSUBACK as u8 => Some(Packet :: UNSUBACK), 
            Packet :: PINGREQ as u8 => Some(Packet :: PINGREQ), 
            Packet :: PINGRESP as u8 => Some(Packet :: PINGRESP), 
            Packet :: DISCONNECT as u8 => Some(Packet :: DISCONNECT), 
            Packet :: AUTH as u8 => Some(Packet :: AUTH), 
            _ => None
        }
    }
}
