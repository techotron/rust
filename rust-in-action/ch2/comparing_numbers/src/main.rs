// Comparisons are enabled by using "traits". The operators which are used a similar to other languages.
// It's impossible to compare numbers of different types. This is a compile time error. 
// In order to compare numbers of a different type,  you must cast them using "as":

fn my_as() {
    let a: i32 = 10;
    let b: u16 = 100;

    if a < (b as i32) {
        println!("Ten is less than one hundred");
    }
}

// Note: The risk of using "as" can cause unexpected behaviour. Eg, 300_i32 as u8 will return 44.



// This enables try_into() to be used on those types which have enabled it (such as u16)
// This is a trait which we're bringing into local scope. 
use std::convert::TryInto;


fn my_try_into() {
    let a: i32 = 10;
    let b: u16 = 100;

    let b_ = b.try_into()
              .unwrap();

    // try_into() returns a Result<T, E> type that provides access to the conversion attempt.
    // The unwrap() method is used to extract the success value

    if a < b_ {
        println!("Ten is less than one hundred");
    }
}




fn main() {
    my_as();
    my_try_into();
}