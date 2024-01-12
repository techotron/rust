// This won't compile because Rust doesn't know which string reference will get returned
//  at compilation time, so it can't determine the lifecycle of the string references being
//  passed into it.
// The error test points us to use an "expected lifetime parameter"
// fn longest_broken(s1: &str, s2: &str) -> &str {
//     if s1.len() > s2.len() {
//         s1
//     } else {
//         s2
//     }
// }

// To fix the above, we can annotate the references with a lifetime. The lifetime
//  parameter must start with an apostrophe '
// NOTE: The convention is to use the name 'a for the first lifetime annotation eg:
// &i32 - just a normal reference without a lifetime annotation
// &'a i32 - a reference with an explicit lifetime
// &'a mut i32 - a mutable reference with an explicit lifetime

// The function signature here is:
//  - Defining a lifetime parameter called 'a
//  - Annotating both parameters with the same lifetime parameter called 'a
//  - The function returns a string slice that will last as long as lifetime 'a
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
