// This commented code here demonstrates why we need to use the move keyword in the closure.
//  The problem is that the closure infers what sort of ownership it needs for v. As println
//  only needs a borrow, that's what Rust tries to do - but because the compiler can't
//  guarantee how long the thread will last and therefore not know if v will still exist
//  by the time the thead has finished, it will not compile.

// use std::thread;
//
// fn main() {
//     let v = vec![1, 2, 3];
//
//     let handle = thread::spawn(|| {
//         println!("Here's a vector: {:?}", v);
//     });
//
//     // Dropping v here to demonstrate what the compiler is warning of.
//     drop(v); // oh no!
//
//     handle.join().unwrap();
// }
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // By using the move keyword here, we TELL the closure to transfer ownership
    //  of v to the the inner block, guaranteeing that it'll exist for as long as
    //  the thread.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
