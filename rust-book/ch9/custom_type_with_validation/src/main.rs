fn main() {
    // This example shows how you could use a custom type in the guessing game from chapter 2
    // The type is declared with a value of i32
    // Type has 2 methods implemented, new and value
    // Guess::new will validate if the value is within bounds and panic if not. If it is, it will
    //  return an instance of Guess
    // Guess::value will return a borrowed instance of self and return an i32. It's known as a getter
    //  This public method is necessary because the value field in the Guess struct is private. It's
    //  important to keep this private so that code using the Guess struct can't set the value field
    //  directly, and bypass the validation we've implemented.
    // We've seen an example of updating a struct field value once's been created in ch7 (file:///Users/edward.snow/git/github.com/rust-lang/book/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}", guess)
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

}