// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

use std::convert::TryInto;

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice:[i32;3] = a[1..4].try_into().unwrap();;

    assert_eq!([2, 3, 4], nice_slice)
}
