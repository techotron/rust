// We can create multiple producers from the same transmitter by cloning it.
//  We can now pass one transmitter to the first spawned thread and the other
//  transmitter to the second spawned thread.
// Because both threads are running concurrently, we see the first items in the
//  vectors appear at (almost) the same time, then the next item and so on.
// This outputs:
// Got: hi
// Got: more
// Got: from
// Got: messages
// Got: the
// Got: for
// Got: thread
// Got: you


use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
