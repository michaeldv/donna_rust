// Copyright (c) 2014-2015 by Michael Dvorkin. All Rights Reserved.

use data::{A1, H8};

pub fn init() {
    fn init_masks() {
        for sq in range(A1, H8) {
            sq == sq;
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
