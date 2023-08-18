// Example of how Rust protects against dangling pointers
// This doens't compile because we're attempting to access a deleted value. The compiler will complain about the value
// "borrowed here after move". 

// This allows the println! macro to print the Cereal enum
#[derive(Debug)]
enum Cereal {
    Barley, Millet, Rice, Rye, Spelt, Wheat
}

fn main() {
    // Initalises an empty vector of Cereal
    let mut grains: Vec<Cereal> = vec![];

    // Adds an item to the grains vector
    grains.push(Cereal::Rye);

    // Deletes grains and it's contents
    drop(grains);

    // Attemps to access the deleted value
    println!("{:?}", grains);
}


