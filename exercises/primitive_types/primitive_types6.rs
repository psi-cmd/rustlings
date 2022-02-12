// primitive_types6.rs
// Use a tuple index to access the second element of `numbers`.
// You can put the expression for the second element where ??? is so that the test passes.
// Execute `rustlings hint primitive_types6` for hints!

#[test]
fn indexing_tuple() {
    let mut numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let mut second = numbers.1;
    numbers.1 = 4;
    assert_eq!((1,4,3), numbers,
        "This is not the 2nd number in the tuple!")
}
