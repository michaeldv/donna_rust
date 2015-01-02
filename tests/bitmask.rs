// Copyright (c) 2014-2015 by Michael Dvorkin. All Rights Reserved.

extern crate rusty_donna;
use rusty_donna::bitmask::Bitmask;

#[test]
fn bitmask_test_010() {
    assert!(Bitmask(0).empty() == true);
    assert!(Bitmask(7).empty() == false);
}

#[test]
fn bitmask_test_020() {
    assert!(Bitmask(0).any() == false);
    assert!(Bitmask(7).any() == true);
}

#[test]
fn bitmask_test_030() {
    assert!(Bitmask(0).count() == 0);
    assert!(Bitmask(8).count() == 1);
    assert!(Bitmask(0xFFFFFFFFFFFFFFFF).count() == 64);
}

#[test]
fn bitmask_test_040() {
    assert!(Bitmask(1).first() == 0);
    assert!(Bitmask(8).first() == 3);
    assert!(Bitmask(0x8000000000000000).first() == 63);
}

#[test]
fn bitmask_test_050() {
    assert!(Bitmask(1).on(0) == true);
    assert!(Bitmask(1).off(0) == false);
    assert!(Bitmask(8).on(3) == true);
    assert!(Bitmask(8).off(3) == false);
    assert!(Bitmask(0x8000000000000000).on(63) == true);
    assert!(Bitmask(0x8000000000000000).off(63) == false);
}

#[test]
fn bitmask_test_060() {
    let mut mask = Bitmask(0b0001);
    assert!(mask.pop() == 0);
    assert!(mask == Bitmask(0b0000));

    mask = Bitmask(0b1110);
    assert!(mask.pop() == 1);
    assert!(mask == Bitmask(0b1100));

    mask = Bitmask(0x8000000000000000);
    assert!(mask.pop() == 63);
    assert!(mask == Bitmask(0));
}

#[test]
fn bitmask_test_070() {
    let mut mask = Bitmask(0);
    assert!(mask.set(0).val() == Bitmask(1).val());
    assert!(mask.clear(0).val() == Bitmask(0).val());
    assert!(mask.set(3).val() == Bitmask(8).val());
    assert!(mask.clear(3).val() == Bitmask(0).val());
    assert!(mask.set(63).val() == Bitmask(0x8000000000000000).val());
    assert!(mask.clear(63).val() == Bitmask(0).val());
}
