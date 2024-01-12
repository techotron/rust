use std::thread;
use std::time::Duration;

fn main() {
    // thread::spawn returns a JoinHandle, which we can call the join() method on, which
    //  will cause the execution to wait for the thread to finish.
    // So even though the main thread loop will finish first, the handle.join() will
    //  ensure that the spawned threads will finish first.
    let handle = thread::spawn(|| {
       for i in 1..10 {
           println!("hi number {} from the spawned thread!", i);
           thread::sleep(Duration::from_millis(1));
       }
    });

    for i in 1..3 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // join() here, blocks the thread from continuing (in this case, continuing would
    //  mean to exit the process because it's the last line).
    handle.join().unwrap();
}
