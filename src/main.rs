//
//  main.rs
//  Rusty Donna chess engine.
//
//  Created by Michael Dvorkin on 25-DEC-2014.
//  Copyright (c) 2014-2015 Michael Dvorkin. All rights reserved.

extern crate rusty_donna;

#[cfg(not(test))]
use rusty_donna::bitmask::Bitmask;
#[cfg(not(test))]
use rusty_donna::moves::Move;

#[cfg(not(test))]
fn main() {
    let zero = Bitmask(0);
    let nonzero = Bitmask(7);

    println!("Hello, world!");
    println!("{}.empty() {}", zero, zero.empty());
    println!("{}.any() {}", zero, zero.any());
    println!("{}.empty() {}", nonzero, nonzero.empty());
    println!("{}.any() {}", nonzero, nonzero.any());
}
