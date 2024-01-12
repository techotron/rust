pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail!");
    }

    // Returning a Result type in the test, allows us to use the question mark operator
    //  within the test. The question mark operator will use the T value or returns the Err
    //  if an error. So in order to use in INSIDE a test, the test itself would have to
    //  have that same return type. This is useful for testing functions where we could use
    //  the question mark operator.
    #[test]
    fn test_with_result_return_type() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
