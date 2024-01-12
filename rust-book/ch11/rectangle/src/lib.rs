#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    // The "tests" module is an inner module and we need to bring the code under "test"
    //  in the outer module into scope of the inner module.
    // Basically, this brings all items from the parent, into the "tests" module scope,
    //  allowing us to create a "Rectangle" object without specifying the full path. Without
    //  calling super here, we would have to use the full path: crate::Rectangle
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 8,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn is_adds_two() {
        // Macro to asserting left equals right. If the assertion fails, it prints the
        //  result to the terminal
        // The assert_ne! macro asserts that's the 2 values do not equal each other.
        // The macros print failed assertions using the debug formatting, so the value
        //  used here MUST be one which implements the PartialEq and Debug traits (which
        //  most do). Any custom structs or enums will need these traits implemented
        //  ourselves, (eg #[derive(PartialEq, Debug)]
        assert_eq!(4, add_two(2));
    }
}