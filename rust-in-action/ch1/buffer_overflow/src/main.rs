// Example of how Rust protects against buffer overflows
// Compiling this will result in an error message along the lines of thread panicked at index out of bounds because 
// the index 4 is out of bounds for the vector fruit

fn main() {
    let fruit = vec!['🥝', '🍌', '🍇'];

    let buffer_overflow = fruit[4];
    assert_eq!(buffer_overflow, '🍉')
}