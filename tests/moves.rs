// Copyright (c) 2014-2015 by Michael Dvorkin. All Rights Reserved.

extern crate rusty_donna;
use rusty_donna::moves::Move;

#[test]
fn move_test_010() {
	assert!(Move::new(1,2) == Move(513))
}
