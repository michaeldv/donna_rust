// Copyright (c) 2014-2015 by Michael Dvorkin. All Rights Reserved.

// Returns row number in 0..7 range for the given square.
pub fn row(square: uint) -> uint {
    square >> 3
}

// Returns column number in 0..7 range for the given square.
pub fn col(square: uint) -> uint {
    square & 7
}

// Returns both row and column numbers for the given square.
pub fn coordinate(square: uint) -> (uint, uint) {
    (row(square), col(square))
}
