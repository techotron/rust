fn main() {
    println!("Hello, world!");

    another_function("my_argument");

    my_expression();

    let my_return_value = my_return_value();
    println!("my_return_value: '{my_return_value}'")
}

fn another_function(param_one: &str) {
    println!("another function here, param_one: '{param_one}'");
}

// Statements vs Expressions
// Statements are instructions that perform an action but don't return a value
// Expressions evaluate to a resultant value
//  let x = 6; is a statement. It does not return a value so let y = (let x = 6); will not compile


fn my_expression() {
    let y = {
        let x = 5;

        // The below line does NOT have a semi-colon. This is intentional. Adding one would change it from an expression to a statement, meaning it wouldn't return a value and cause an error at compilation time
        x + 1
    };

    println!("The result of the y expression is: '{y}'")
}

fn my_return_value() -> i32 {
    // The return keyword could be used or the last value in the expression would be used implicitly
    5
}