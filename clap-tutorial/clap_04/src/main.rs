// use clap::{arg, command};
//
// fn main() {
//     let matches = command!()
//         // Required arg with enum
//         .arg(
//             // <MODE> is a positional arg and allows us to match on "MODE" and the <> make it a required arg
//             // [MODE] is a positional arg and allows us to match on MODE but the [] make it optional
//             arg!(<MODE>)
//                 .help("What mode to run the program in")
//                 .value_parser(["fast", "slow"])
//         )
//         .get_matches();
//
//     // It's safe to call expect() (or unwrap() if we wanted) here because the argument is required
//     match matches
//         .get_one::<String>("MODE")
//         .expect("'MODE' is required and parsing will fail if its missing")
//         .as_str()
//     {
//         "fast" => {
//             println!("Hare");
//         }
//         "slow" => {
//             println!("Tortoise");
//         }
//         _ => unreachable!(),
//     }
// }


use clap::{arg, builder::PossibleValue, command, value_parser, ValueEnum};

// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Clone, Copy)]
enum Mode {
    Fast,
    Slow,
}

// Can also be derived with feature flag `derive`
impl ValueEnum for Mode {
    fn value_variants<'a>() -> &'a [Self] {
        &[Mode::Fast, Mode::Slow]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Mode::Fast => PossibleValue::new("fast").help("Run swiftly"),
            Mode::Slow => PossibleValue::new("slow").help("Crawl slowly but steadily"),
        })
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}


fn http_port(s: &str) -> Result<u16, String> {
    let valid_http_ports: Vec<usize> = vec![80, 443];
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a valid port number"))?;

    if valid_http_ports.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!("not an allowed http port"))
    }
}

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(
            arg!(<MODE>)
                .help("What mode to run the program in")
                .value_parser(value_parser!(Mode)),
        )
        .arg(
            arg!(<PORT>)
                .help("Network port to use")
                .value_parser(value_parser!(u16).range(1..)),
        )
        // Use a custom function to validate inputs
        .arg(
            arg!(<HTTP_PORT>)
                .help("Port for http traffic")
                .value_parser(http_port),
        )
        .get_matches();

    let port: u16 = *matches
        .get_one::<u16>("PORT")
        .expect("'PORT' is required and parsing will fail if it is missing");

    // Note, it's safe to call unwrap() because the arg is required
    match matches
        .get_one::<Mode>("MODE")
        .expect("'MODE' is required and parsing will fail if its missing")
    {
        Mode::Fast => {
            println!("Hare");
        }
        Mode::Slow => {
            println!("Tortoise");
        }
    }

    println!("Port: {port}");
}