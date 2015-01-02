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
    println!("move {}", Move::new(1,2));

    let a = Bitmask(1);
    let b = Bitmask(2);
    let c: Bitmask = a | b;
    println!("a | b = {}", c);
    println!("a << 3 = {}", a << 3);
    println!("1 = {}", Bitmask(1) != Bitmask(0));
    println!("F = {}", Bitmask(0xFFFFFFFFFFFFFFFF));
}
