pub enum Duplicate
{
    Duplicate,
    Original
}

impl Duplicate
{
    pub fn from_bool(bit : bool) -> Duplicate
    {
        match bit
        {
            true => Duplicate::Duplicate,
            false => Duplicate::Original
        }
    }
}