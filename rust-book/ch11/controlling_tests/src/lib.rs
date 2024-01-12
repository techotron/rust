use std::{thread, time};

pub fn sleep_for_one_second() {
    thread::sleep(time::Duration::from_secs(1));
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

// By default, cargo will run the tests in parallel in different threads

// Parallelism
// cargo test -- --test-threads=1 will limit the tests to a single thread, thereby
//  making them sequential

// Output
// To show output, cargo test -- --show-output

// Targeting tests
// cargo test one_hundred (run a single test with given name)
// cargo test add (run tests with "add" in the name)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    // This test is ignored unless cargo test -- --include-ignored is run
    #[test]
    #[ignore]
    fn expensive_test() {
        sleep_for_one_second();
    }
}