use std::fs::File;
use std::io::ErrorKind;
use std::error::Error;
use std::fs::remove_file;
use std::io::{self, Read};

fn main() {
    panic_on_error_match();
    panic_on_error_match_using_closures();
    // panic_on_error_using_unwrap();
    panic_on_error_with_expect();
    propagating_errors_example_function();
    propagating_errors_with_question_mark();

    clean_up();
}

fn clean_up() {
    let _ = remove_file("hello.txt");
}

fn panic_on_error_match() {
    // File::open has a return type of Result<T, E> where T is the success return value (a file handle) or E which is an Err (with some useful information about the error)
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        // The error returned is a struct provided by the standard library which has a method called kind()
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn panic_on_error_match_using_closures() {
    // More about closures in a later chapter but this achieves the same result as the above function but using closures instead of match
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

#[warn(dead_code)]
fn panic_on_error_using_unwrap() {
    // unwrap will return the T value in Result or will call the panic macro for us if it's an error
    let _greeting_file = File::open("not-exist.txt").unwrap();
}

fn panic_on_error_with_expect() {
    // expect will do the same as unwrap but will pass the parameter used to the panic macro
    let _greeting_file = File::open("not-exist.txt")
        // The text here will appear in the panic message displayed
        .expect("not-exist.txt should be included in this project");
}

fn propagating_errors_example_function() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        // Grab the T value of Result, bind to "file" and return it. This is the value that the
        //  mutable variable "username_file" is given
        Ok(file) => file,
        // Grab the E value of Result and bind to "e" and return an Err type with "e" as the value
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    // Read the contents of the file into the mutable username variable.
    match username_file.read_to_string(&mut username) {
        // If success, return the username from the file that's now in the username
        //  variable, wrapped in "Ok"
        Ok(_) => Ok(username),

        // No need to explicitly say "return" here, because it's the last expression
        //  in the function
        Err(e) => Err(e),
    }
}

fn propagating_errors_with_question_mark() -> Result<String, io::Error> {
    // The ? after a Result value is defined to work in the same way as the match expressions
    //  in the above example.
    // If the Result is an Ok, the value of the Ok will get returned from the expression, else the
    //  Err will be returned.
    // The difference between the ? and match patterns is that the values that have the ? operator
    //  called on them, go through the "from" function defined in the From trait, which is used to
    //  convert values from one type to another. It will try to convert it to the same type from the
    //  Result type from the function call.
    // This automatic conversion is useful, for example, if we wanted to return a custom Error type called
    //  OurError, we could define the type, imply the io::Error trait for it and the ? operator would
    //  attempt to convert the Err returned from the File::open, into OurError
    // Note: The ? operator can only be used for functions which return the Result type, otherwise it'll fail
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// This is the same function as above but even less verbose, using chaining and the ? operator
// Note: Reading in a file is pretty common, so the std library has a function to call which does this
//  fs::read_to_string("hello.txt")
// This would return a Result<String, io::Error>
fn propagating_errors_with_question_mark_with_chaining() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// Box<dyn Error> type is a "trait object". It can be used to mean "any kind of error".
//  This allows us to use the ? operator directly inside the main function.
fn example_main_with_return() -> Result<(), Box<dyn Error>> {
    let _greeting_file = File::open("hello.txt")?;

    Ok(())
}