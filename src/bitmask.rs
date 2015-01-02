// Copyright (c) 2014-2015 by Michael Dvorkin. All Rights Reserved.

use std::fmt;
use std::char::{from_digit};

#[deriving(Eq, PartialEq, Copy)]
pub struct Bitmask(pub u64); // <-- Newtype pattern.

static DE_BRUIJN: [uint; 64] = [
     0, 47,  1, 56, 48, 27,  2, 60,
    57, 49, 41, 37, 28, 16,  3, 61,
    54, 58, 35, 52, 50, 42, 21, 44,
    38, 32, 29, 23, 17, 11,  4, 62,
    46, 55, 26, 59, 40, 36, 15, 53,
    34, 51, 20, 43, 31, 22, 10, 45,
    25, 39, 14, 33, 19, 30,  9, 24,
    13, 18,  8, 12,  7,  6,  5, 63
];

impl BitAnd<Bitmask, Bitmask> for Bitmask {
    fn bitand(self, rhs: Bitmask) -> Bitmask {
        Bitmask(self.0 & rhs.0)
    }
}

impl BitOr<Bitmask, Bitmask> for Bitmask {
    fn bitor(self, rhs: Bitmask) -> Bitmask {
        Bitmask(self.0 | rhs.0)
    }
}

impl BitXor<Bitmask, Bitmask> for Bitmask {
    fn bitxor(self, rhs: Bitmask) -> Bitmask {
        Bitmask(self.0 ^ rhs.0)
    }
}

impl Shr<uint, Bitmask> for Bitmask {
    fn shr(self, rhs: uint) -> Bitmask {
        Bitmask(self.0 >> rhs)
    }
}

impl Shl<uint, Bitmask> for Bitmask {
    fn shl(self, rhs: uint) -> Bitmask {
        Bitmask(self.0 << rhs)
    }
}

impl Bitmask {
    pub fn empty(&self) -> bool {
        return *self == Bitmask(0)
    }

    pub fn any(&self) -> bool {
        return *self != Bitmask(0)
    }

    // Returns number of bits set.
    pub fn count(&self) -> uint {
        let mut mask = self.val();
        mask -= (mask >> 1) & 0x5555555555555555;
        mask = ((mask >> 2) & 0x3333333333333333) + (mask & 0x3333333333333333);
        mask = ((mask >> 4) + mask) & 0x0F0F0F0F0F0F0F0F;
        return ((mask * 0x0101010101010101) >> 56) as uint
    }

    // Finds least significant bit set (LSB) in non-zero bitmask. Returns uint
    // in 0..63 range.
    pub fn first(&self) -> uint {
        let mask = self.val();
        DE_BRUIJN[(((mask ^ (mask - 1)) * 0x03F79D71B4CB0A89) >> 58) as uint]
    }

    pub fn on(&self, offset: uint) -> bool {
        return (self.val() & (1 << offset)) != 0
    }

    pub fn off(&self, offset: uint) -> bool {
        return !self.on(offset)
    }

    // Finds *and clears* least significant bit set (LSB) in non-zero bitmask.
    // Returns uint in 0..63 range.
    pub fn pop(&mut self) -> uint {
        let mask = self.val();
        let magic = mask - 1;
        *self = Bitmask(mask & magic);
        DE_BRUIJN[(((mask ^ magic) * 0x03F79D71B4CB0A89) >> 58) as uint]
    }

    // Sets a bit at given offset.
    pub fn set(&mut self, offset: uint) -> &mut Bitmask {
        *self = Bitmask(self.val() | (1 << offset));
        self
    }

    // Clears a bit at given offset.
    pub fn clear(&mut self, offset: uint) -> &mut Bitmask {
        *self = Bitmask(self.val() & ((1 << offset) ^ 0xFFFFFFFFFFFFFFFF));
        self
    }

    pub fn val(&self) -> u64 {
        (*self).0
    }
}

impl fmt::Show for Bitmask {
    #[allow(unused_must_use)] // <-- To avoid try!(write!()) nonsense.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "  a b c d e f g h  0x{:016X}", self.val());
        for row in range(0u, 8u).rev() {
            write!(f, "\n{}", from_digit(row + 1, 10).unwrap());
            for col in range(0u, 8u) {
                write!(f, " ");
                let offset = (row << 3) + col;
                if self.on(offset) {
                    write!(f, "\u{2022}");
                } else {
                    write!(f, "\u{22C5}");
                }
            }
        }
        write!(f, "\n")
    }
}
