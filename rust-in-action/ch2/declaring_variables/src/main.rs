// Each of the below is a valid way to declare a variable

fn main() {
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));

    println!("(a + b) + (c + d) = {}", e);
}


// Note: no "return" keyword is needed. Functions return the last expression's result
fn add(i: i32, j: i32) -> i32 {
    i + j
    // Note: no semicolon. If there was one, the function would return "()" (unit type) rather than an i32
}
