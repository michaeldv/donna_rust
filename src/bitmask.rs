use std::fmt;

#[deriving(Eq, PartialEq, Copy)]
pub struct Bitmask(pub u64);

impl Bitmask {
    pub fn empty(&self) -> bool {
        return *self == Bitmask(0)
    }

    pub fn any(&self) -> bool {
        return *self != Bitmask(0)
    }

    pub fn as_u64(&self) -> u64 {
        let Bitmask(cast) = *self;
        return cast;
    }
}

impl fmt::Show for Bitmask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_u64())
    }
}
