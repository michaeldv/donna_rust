// Copyright (c) 2014-2015 by Michael Dvorkin. All Rights Reserved.

// Returns row number in 0..7 range for the given square.
pub fn row(square: usize) -> usize {
    square >> 3
}

// Returns column number in 0..7 range for the given square.
pub fn col(square: usize) -> usize {
    square & 7
}

// Returns both row and column numbers for the given square.
pub fn coordinate(square: usize) -> (usize, usize) {
    (row(square), col(square))
}
