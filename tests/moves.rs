extern crate rusty_donna;
use rusty_donna::moves::Move;

#[test]
fn move_test_010() {
	assert!(Move::new(1,2) == Move(513))
}
