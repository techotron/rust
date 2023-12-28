use crate::garden::vegetables::Asparagus;

// This tells the compiler to include the code found in src/garden.rs
// By default, the compiler is looking for the module's code in:
// - src/garden.rs
// - src/garden/mod.rs
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);
}
