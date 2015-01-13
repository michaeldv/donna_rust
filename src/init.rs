// Copyright (c) 2014-2015 by Michael Dvorkin. All Rights Reserved.

use std::cmp::max;
//use std::mem::zeroed; unsafe { zeroed() }
use std::num::SignedInt;
use bitmask::Bitmask;
use utils::{row, col, coordinate};

// Distance between two squares.
static mut _distance: [[usize; ..64]; ..64] = [[0; ..64]; ..64];
static mut _king_moves: [Bitmask; ..64] = [Bitmask(0); ..64];
static mut _knight_moves: [Bitmask; ..64] = [Bitmask(0); ..64];

#[inline(always)]
pub fn distance(from: usize, to: usize) -> usize {
    unsafe { _distance[from][to] }
}

#[inline(always)]
pub fn king_moves(square: usize) -> Bitmask {
    unsafe { _king_moves[square] }
}

#[inline(always)]
pub fn knight_moves(square: usize) -> Bitmask {
    unsafe { _knight_moves[square] }
}

pub fn init() {

    fn init_masks() {
        for sq in range(0, 64) {
            let (row, col) = coordinate(sq);

            for i in range(0, 64) {
                let (r, c) = coordinate(i);
                let absrow = ((r - row) as isize).abs();
                let abscol = ((c - col) as isize).abs();

                unsafe {
                    _distance[sq][i] = max(absrow, abscol) as usize;
                }

                if i == sq || ((i - sq) as isize).abs() > 17 {
                    continue; // No king or knight can reach that far.
                }

                if (absrow == 2 && abscol == 1) || (absrow == 1 && abscol == 2) {
                    unsafe { _knight_moves[sq].set(i); }
                }

                if absrow <= 1 && abscol <= 1 {
                    unsafe { _king_moves[sq].set(i); }
                }

            }
        }

        println!("init_masks...");
    }

    fn init_arrays() {
        println!("init_arrays...");
    }

    fn init_pst() {
        println!("init_pst...");
    }

    fn init_material() {
        println!("init_material...");
    }

    init_masks();
    init_arrays();
    init_pst();
    init_material();
}
