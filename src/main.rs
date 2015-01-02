//  main.rs
//  Rusty Donna chess engine.
//
//  Created by Michael Dvorkin on 25-DEC-2014.
//  Copyright (c) 2014-2015 Michael Dvorkin. All rights reserved.

extern crate rusty_donna;

#[cfg(not(test))]
use rusty_donna::init;
#[cfg(not(test))]
use rusty_donna::bitmask;
#[cfg(not(test))]
use rusty_donna::moves;

#[cfg(not(test))]
fn main() {
    let zero = bitmask::Bitmask(0);
    let nonzero = bitmask::Bitmask(7);

    println!("Hello, world!");
    println!("{}.empty() {}", zero, zero.empty());
    println!("{}.any() {}", zero, zero.any());
    println!("{}.empty() {}", nonzero, nonzero.empty());
    println!("{}.any() {}", nonzero, nonzero.any());
    println!("move {}", moves::Move::new(1,2));

    let a = bitmask::Bitmask(1);
    let b = bitmask::Bitmask(2);
    let c: bitmask::Bitmask = a | b;
    println!("a | b = {}", c);
    println!("a << 3 = {}", a << 3);
    println!("1 = {}", bitmask::Bitmask(1) != bitmask::Bitmask(0));
    println!("F = {}", bitmask::Bitmask(0xFFFFFFFFFFFFFFFF));

    init::init();
}
