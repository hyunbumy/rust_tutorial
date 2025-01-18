// Each file in the `tests` directory is a separate crate so we need to bring the library into each
// test crate's scope.
use tests::add;

mod common;

#[test]
fn add_success() {
    common::setup();

    let result = add(2, 2);

    assert_eq!(result, 4);
}

