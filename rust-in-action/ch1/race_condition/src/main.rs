// Example of how Rust protects against Race conditions
// In this example, 2 threads are created, both of which will attempt to change the value of data non-deterministically
// because the scheduling of the threads is determined by the OS rather than the program. 
// Rust doesn't allow multiple places to have write access to data. In this example, we're attempting to give write access
// in 3 places: the main thread and the 2 spawned threads. 

// This brings multi-threading into scope
use std::thread;

fn main() {
    let mut data = 100;

    // thread::spawn() takes a closure as an argument, denoted by the double pipe and curly braces. The closure is the code that will be executed by the thread
    thread::spawn(|| { data = 500; });
    thread::spawn(|| { data = 1000; });

    println!("{}", data);
}