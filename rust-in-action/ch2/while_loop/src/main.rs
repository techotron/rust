use std::time::{ Duration, Instant };

// How fast the computer can increment a counter.
fn main() {
    let mut count = 0;                      // We're mutating count in the loop
    let time_limit = Duration::new(1, 0);   // 1 second
    let start = Instant::now();             // Time "now" from the system's clock

    while (Instant::now() - start) < time_limit { // An Instant minus an Instant returns a Duration
        count += 1;
    }
    println!("{}", count);
}