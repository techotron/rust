use clap::{command, arg, value_parser, Arg, ArgAction, Command};

fn main() {
    // Append positional arguments and collect into the args variable as a Vec
    // let matches = command!()
    //     .next_line_help(true)
    //     .arg(Arg::new("name").action(ArgAction::Append))
    //     .get_matches();
    //
    // let args = matches
    //     .get_many::<String>("name")
    //     .unwrap_or_default()
    //     .map(|v| v.as_str())
    //     .collect::<Vec<_>>();
    // println!("name: {:?}", &args);


    let matches = command!()
        .arg_required_else_help(true)
        .subcommand_required(false)

        // A subcommand can have separate metadata than the app
        .subcommand(
            Command::new("add")
                .about("Adds a thing")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name"),
                ),
        )
        // You can name the arguments with long and short syntax:
        // <cli_app -n eddy or --name eddy or --name=eddy or -neddy
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .action(ArgAction::Append),
        )
        // Change the action to SetTrue (makes the arg a boolean flag)
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue),
        )
        // Count the number of times the arg has been called
        .arg(
            Arg::new("verbose-count")
                .long("verbose-count")
                .action(ArgAction::Count),
        )
        // Default value
        .arg(
            arg!(-p --port <PORT>)
                .value_parser(value_parser!(u16))
                .default_value("2020"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => println!(
            "add was used, name is: {:?}",
            sub_matches.get_one::<String>("name")
        ),
        // If subcommand_required == true, then the default arm could be this:
        // _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`")
        _ => {
            let is_verbose = matches.get_flag("verbose");
            if is_verbose {
                println!("verbose: {:?}", is_verbose);
            }

            println!("name: {:?}", matches.get_one::<String>("name"));
            println!("verbose-count: {:?}", matches.get_count("verbose-count"));
            println!(
                "Port: {:?}",
                matches
                    .get_one::<u16>("port")
                    .expect("default ensures there is always a value...")
            )
        }
    }
}