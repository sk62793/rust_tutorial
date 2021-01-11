extern crate adder;

mod common;

#[test]
fn it_adds_two() {
    common::setUp();
    assert_eq!(4, adder::add_two(2));
}
