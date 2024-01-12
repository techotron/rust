// This is similar code as the other channels example but demonstrates how to send
//  and receive multiple values over the channel.
// A difference here though, is that we're not calling recv in the main thread anymore.
//  Instead, we're treating rx as an iterator. When the iterator ends, the channel is
//  closed.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
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
