fn main() {
    without_borrowing();
    using_a_reference();
    mutable_references();
}

fn without_borrowing() {
    // One of the problems with ownership is that we may want to pass in a value to a function which we'll later need.
    //  However, once it's moved into the function, it goes out of scope in the outer function.  This could be retrieved by having the inner function return a tuple and returning the string back that way like the below example. A better pattern however is to use Rust's borrowing feature.
    let s1 = String::from("Hello, World!");
    let (s2, length) = returning_a_tuple(s1);
    println!("s2 = {}, length = {}", s2, length);
}

fn returning_a_tuple(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}


fn using_a_reference() {
    // Here, calculate_length takes in a reference so the ownership of s1 remains with the outer function's scope and can be called after the calculate_length() function is called.
    // The ampersand represents a "reference" which in Rust verbiage is also known as "borrowing".
    // You can't change the value of a borrowed variable.

    // Under the hood, a reference is a stack item with a pointer to the owner's pointer to the memory on the heap.
    let s1 = String::from("Hello, World!");
    let length = calculate_length(&s1);
    println!("s1 = {}, length = {}", s1, length);
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn mutable_references() {
    // You can't change borrowed values by default but can can make some tweaks to enable you to do so:
    let mut s = String::from("hello");
    change(&mut s);
    println!("mutable reference: {}", s);

    // A big caveat to this is that you can't have any other references to the same value. The reason for this caveat is to prevent a data race where the same address space is attempted to be modified.
    // This will error:
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("r1 = {}, r2 = {}", r1, r2);

    // You could, however move the reference into another scope:
    {
        let r1 = &mut s;
        println!("r1 inner scope: {}", r1);
    }
    let r2 = &mut s;
    println!("r2 outer scope: {}", r2);

    // Rust enforces similar rules when combining mutable and immutable references. You can either have one mutable reference or any number of immutable references.
    // let r3 = &s; // no problem
    // let r4 = &s; // no problem
    // let r5 = &mut s; // BIG PROBLEM

    // A references scope will last from when it's introduced to the last time its used:

    let mut s2 = String::from("hello");
    let r6 = &s2; // no problem
    let r7 = &s2; // no problem
    println!("r6 = {}, r7 = {}", r6, r7);

    let r8 = &mut s2; // no problem because r6 and r7 are not used after this
    println!("r8 = {}", r8);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}