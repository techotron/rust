// mpsc stands for Multiple Producer, Single Consumer. Rust's standard lib implements
//  channels that can have multiple sending ends but only one receiving end.
use std::sync::mpsc;
use std::thread;

fn main() {
    // Transmitter // Receiver
    let (tx, rx) = mpsc::channel();

    // Using move here, will ensure that tx and val will transfer ownership to the
    //  closure.
    thread::spawn(move || {
        let val = String::from("hi");
        // Send the string. send() returns a Result<T, E>. Unwrap is used for the demo.
        //  If the receiver had been dropped and there's nowhere to send the value, then
        //  the Result from send() would have an Err in it. Unwrap here will panic in that
        //  case.
        tx.send(val).unwrap();
        // At this point, the ownership of val has been moved down the channel and now belongs
        // to the receiver. So the following line for example, would cause the compiler to fail:
        // println!("val is {}", val);
    });

    // recv() will block the thread and wait until a value is sent down the channel.
    //  An alternative is try_recv(), which is useful if this thread has other work to
    //  do while waiting for messages. Eg, write a loop which calls try_recv(), handles
    //  the message if available or does other work.
    // try_recv() does not block. It will return a Result<T, E>. If there is no message,
    //  then Result will be an Err.
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
