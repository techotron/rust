use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    // Here, we're saving the closure as a variable to be called later on.
    // Closures don't typically need type annotations but we can explicitly add them
    //  in the same way you would for a normal function signature.
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, to {} pushups!", expensive_closure(intensity));
        println!("Today, to {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");

        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

// This function demonstrates how the compiler can determine that the list variable
//  is borrowed and immutably used.
fn closures_and_borrowing() {
    let list = vec![1, 2, 3];
    println!("[closures_and_borrowing] Before defining closure: {:?}", list);

    // The compiler will infer whether list will be borrowed mutably, immutably or
    //  taken ownership of it. As list is used elsewhere in the function body, it's
    //  able to determine that we want list to be borrowed immutably.
    let only_borrows = || println!("[closures_and_borrowing] From closure: {:?}", list);

    println!("[closures_and_borrowing] Before calling closure: {:?}", list);
    only_borrows();
    println!("[closures_and_borrowing] After calling closure: {:?}", list);

    // Note: all print lines here will output the same values for list.
}

// This function demonstrates how the compiler can determine that the list variable
//  is borrowed and mutably changed.
fn closures_and_borrowing2() {
    let mut list = vec![1, 2, 3];
    println!("[closures_and_borrowing2] Before defining closure: {:?}", list);

    // At the closure definition here, the compiler captures a mutable reference to list
    //  In other words, after this line, list is a "mutable borrow"
    let mut borrows_mutably = || list.push(7);

    // This line would cause the compiler to complain if uncommented because we'd be attempting
    //  to borrow something which is already mutably borrowed (which isn't allowed).
    // println!("[closures_and_borrowing2] Before calling closure: {:?}", list);

    borrows_mutably();
    println!("[closures_and_borrowing2] After calling closure: {:?}", list);
}

// This function demonstrates how you can force the closure to take ownership, even if
//  it's not strictly necessary. This is a useful technique when you want a thread to
//  take ownership of the data it's handling.
fn closures_and_force_take_ownership() {
    let list = vec![1, 2, 3];
    println!("[closures_and_force_take_ownership] Before defining closure: {:?}", list);

    // We spawn a new thread and pass a closure to run as an argument
    // The move keyword explicitly tells the closure to take ownership of list
    //  (even though only an immutable reference isn't necessary to print
    //  the contents out).
    // We needed to use move here because if the main thread maintained ownership
    //  of list and finished before the spawned thread, then list would have been
    //  dropped before it was used by the spawned thread, therefore the compiler
    //  requires it to be moved.
    thread::spawn(move || println!("[closures_and_force_take_ownership] From thread: {:?}", list))
        .join()
        .unwrap();
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
    closures_and_borrowing();
    closures_and_borrowing2();
    closures_and_force_take_ownership();
}
