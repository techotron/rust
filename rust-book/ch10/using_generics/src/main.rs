// Finds the largest number in a vector of i32 types
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


// Finds the largest char in a vector of char types
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


// This function signature is how you define a generic function.
// NOTE: It doesn't compile because the logic requires the T type to implement the
//  std::cmp::PartialOrd trait so that it can order the elements. This will be resolved
//  in another function
// The signature reads as:
//   We read this definition as: the function largest is generic over some type T.
//   This function has one parameter named list, which is a slice of values of type T.
//   The largest function will return a reference to a value of the same type T.
// fn generic_largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }


// Generic struct definition
#[warn(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

// Generic method definition. This is a generic method on the Point struct with
//  a generic type
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

// This method will only apply to instances of Point which have a type of f32
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
    let _integer_and_float = Point { x: 5, y: 4.0 };
}
