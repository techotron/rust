fn main() {
    // This creates a String type from a string literal. The String type manages data stored on the heap, therefore it can store an amount of text that is unknown by us at compile time
    let s = String::from("Hello");
    println!("{}, World!", s);

    // The String type CAN be mutated if instantiated as such
    let mut ss = String::from("Hello");
    ss.push_str(", World!");
    println!("{}", ss);

    move_string();
    clone_string();
    copy_stack_data();
    ownership();
    returning_values();
}

fn move_string() {
    // Under the covers, the String type is made up of 3 parts; a pointer, length and capacity and is stored on the stack
    // - Pointer -> to the memory address on the heap where the data is stored
    // - Length -> total amount of memory in bytes the contents is currently using
    // - Capacity -> total amount of memory in bytes that the String has received from the allocator.

    let s1 = String::from("hello");
    let s2 = s1;

    // When we assign s1 to s2, the String data is copied (the pointer, length and capacity) from the stack. So s2 is pointing to the same memory address as s1.
    // This is a problem because when s1 and s2 go out of scope, Rust will call the "drop" function (to clean up the heap memory), it will try to free up the same memory address twice (called a double free error).
    // Rust addresses this problem by considering s1, no longer valid once "let s2 = s1;" has been executed. This is known in Rust as a "move": s1 was moved to s2.

    // For example, this line would error:
    // println!("{}", s1);
}


fn clone_string() {
    // To "deeply" copy the data on the heap, you can use the clone() method from the String type:
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2)
}

fn copy_stack_data() {
    // This example uses integers which are stored on the stack because they're a known size:
    let x = 5;
    let y = x;

    // Unlike with data that's on the heap, this code will work, because the stack data is copied.
    println!("x = {}, y = {}", x, y)
}

fn ownership() {
    let s = String::from("hello");
    take_ownership(s); // The value of s is moved into the function so it's no longer valid after this function is called.
    // This will error:
    // println!("s = {}", s)

    let x = 5;
    makes_copy(x); // x is i32 which uses the Copy trait. This means it's copied from the stack (along with the value) therefore it can be used again

    // This will not error:
    println!("x = {}", x)
}


fn take_ownership(some_string: String) {
    println!("{}", some_string);
} // "some_string" goes out of scope here and drop is called, freeing the backing memory

fn makes_copy(some_int: i32) {
    println!("{}", some_int)
} // "some_int" goes out of scope but nothing special happens


fn returning_values() {
    let s1 = gives_ownership(); // gives_ownership() moves it's return value into s1

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back which also moves its return value into s3
} // Here, s1 and s3 go out of scope. s2 was moved into takes_and_gives_back so nothing extra happens

fn gives_ownership() -> String {
    let s = String::fomr("new string from gives_ownership()");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}