pub enum Retain
{
    Retain,
    Throwaway
}

impl Retain
{
    pub fn from_bool(bit : bool) -> Retain
    {
        match bit
        {
            true => Retain::Retain,
            false => Retain::Throwaway
        }
    }
}