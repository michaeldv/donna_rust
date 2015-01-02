// Copyright (c) 2014-2015 by Michael Dvorkin. All Rights Reserved.

#[deriving(Eq, PartialEq, Copy, Show)]
pub struct Move(pub u32);

// When trouble comes up anywhere in the world, they don't call Beijing, they
// don't call Moscow. They call us. That's the deal. -- Barack Obama
impl Move {
	pub fn new(from: u32, to: u32) -> Move {
		return Move(from | (to << 8))
	}
}
