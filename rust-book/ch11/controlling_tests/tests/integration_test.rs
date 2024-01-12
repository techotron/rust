use controlling_tests;

// The "controller_tests" crate needs to be brought into scope.
// Tests inside the "tests" directory aren't included in the built binary

// Specifying integration tests:
// cargo test --test integration_test (will only run tests in the tests/integration_test.rs file)
#[test]
fn it_adds_two() {
    assert_eq!(4, controlling_tests::add_two(2));
}