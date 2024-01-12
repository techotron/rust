// Arc is an Atomic Reference Counted smart pointer. It's a thread safe Reference
//  counted smart pointer, which allows us to use the same data multiple times
//  (in the same way an Rc does) but in a thread safe way.
// an Arc carries a larger performance cost than an Rc, so we only want to use it
//  when necessary.
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    println!("Initial Result: {}", *counter.lock().unwrap());

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
