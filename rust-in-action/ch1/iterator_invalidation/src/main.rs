// Example of iterator invalidation
// The compiler will throw some keywords in the error like "borrow", "move", "trait".

fn main() {
    let mut letters = vec!["a", "b", "c", "d", "e"];

    for letter in letters {
        println!("{}", letter);

        // Copies each letter an appends to the end of letters (attempting to change the iterator while iterating)
        letters.push(letter.clone());
    }
}