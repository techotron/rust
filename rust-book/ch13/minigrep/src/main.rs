use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // unwrap_or_else() will either return the Ok value or in the case of an Err, it will
    //  run the method using the Err value. This method is called a closure.
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Unlike our error handling of Config::build() above, we don't want to capture the
    //  Ok value of run because it's an empty unit. So we can use "if let" which will
    //  run the inner code if run returns an Err type, otherwise it'll silently continue.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
