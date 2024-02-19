use clap::arg;
// use clap::Command;
use clap::command;
use clap::ArgAction;

fn main() {
    // Manually enter in app metadata and arguments using Command::new()
    // let matches = Command::new("MyApp")
    //     .version("1.0")
    //     .about("Does awesome things")
    //     .arg(arg!(--two <VALUE>).required(true))
    //     .arg(arg!(--one <VALUE>).required(true))
    //     .get_matches();

    // Use the command! macro to get this data from Cargo.toml (requires the cargo feature with the clap crate)
    // let matches = command!()
    //     .arg(arg!(--two <VALUE>).required(true))
    //     .arg(arg!(--one <VALUE>).required(true))
    //     .get_matches();

    // Use Command methods to change the application level behaviour...
    let matches = command!()
        .next_line_help(true)
        .arg(arg!(--two <VALUE>).required(true).action(ArgAction::Set)) // ArgAction::Set is the default action but demonstrates how to use it
        .arg(arg!(--one <VALUE>).required(true).action(ArgAction::Set))
        .get_matches();

    println!(
        "two: {:?}",
        matches.get_one::<String>("two").expect("required")
    );
    println!(
        "one: {:?}",
        matches.get_one::<String>("one").expect("required")
    );
}