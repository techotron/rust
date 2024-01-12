use std::error::Error;
use std::fs;
use std::env;

// Box is a trait object. Box<dyn Error> will return a type that implements the Error
//  trait.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // The question mark operator will return the Err value (if it errored) which allows
    //  the calling function to handle the error.
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // impl Iterator<Item = String> tells the function to expect an iterator for args.
    //  The documentation for std::env::Args states that the type of iterator returned
    //  implements the Iterator trait and returns String values.
    // This is expressing that args has a generic type with the trait bounds of:
    //  Iterator<Item = String>, so in effect, args here can be any type which implements
    //  the Iterator type and returns String items.
    // Because we're taking ownership over args, we'll be mutating it by iterating over it
    //  (because we're consuming the items as we iterate over it).
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // The first item in the args iterator is the name of the program. We can ignore that so
        //  call next() here to consume it and move onto the first parameter we're interested
        //  in.
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case
        })
    }
}

// This signature is telling the compiler that the returned data will live as long
//  as the data passed into it via the contents argument.
// As we're returning references to the data we're inputting, we have to tell the compiler
//  that the returned data needs to live as long as the reference we're using. The annotations
//  don't change the lifetime but allows the borrow checker to make sure we're not introducing
//  a bug.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
    // let query = query.to_lowercase();
    // let mut results = Vec::new();
    //
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    //
    // results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

}