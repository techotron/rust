pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn new2(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The should panic annotation tells the test to expect (and therefore pass) if it
    //  sees a panic. It needs to be after the #[test] annotation and before the function
    //  signature.
    // The problem with this is that the test will pass even if the function panics for some
    //  other reason. The next one demonstrates how we can make it more precise.
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // This test will ensure that the panic message contains the substring value defined
    //  in the "expected" argument.
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100_2() {
        Guess::new2(200);
    }
}