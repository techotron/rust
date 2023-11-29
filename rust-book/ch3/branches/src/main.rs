fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("The number is divisible by 4");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3");
    } else if number % 2 == 0 {
        println!("The number is divisible by 2");
    } else {
        println!("The number is not divisible by 4, 3 or 2");
    }

    // if is an expression so we can use it to assign a value to a variable
    let condition = true;

    // The expressions within the if need to return the same type because we're setting "my_number" to the value and variables can only be a single type
    let my_number = if condition { 5 } else { 6 };

    println!("The value of my_number is: {my_number}")
}