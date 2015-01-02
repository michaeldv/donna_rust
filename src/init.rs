// Copyright (c) 2014-2015 by Michael Dvorkin. All Rights Reserved.

use std::cmp::max;
use std::num::SignedInt;
use utils::{row, col, coordinate};

// Distance between two squares.
static mut _distance: [[uint, ..64], ..64] = [[0, ..64], ..64];

#[inline(always)]
pub fn distance(from: uint, to: uint) -> uint {
    unsafe { _distance[from][to] }
}

pub fn init() {

    fn init_masks() {
        for sq in range(0, 64) {
            let (row, col) = coordinate(sq);

            for i in range(0, 64) {
                let (r, c) = coordinate(i);
                unsafe {
                    _distance[sq][i] = max(((row - r) as int).abs(), ((col - c) as int).abs()) as uint;
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
