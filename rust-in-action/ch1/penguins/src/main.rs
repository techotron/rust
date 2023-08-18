fn main() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        // Skip header row and lines with only whitespace
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // A Vec is a vector - a collection type which can expand dynamically. The underscore instructs the compiler to infer the type of the elements.
        let fields: Vec<_> = record
            .split(",")
            // We're passing a closure/anonymous function to the map function: "for each field, trim it". The |field| is the argument to the closure, the value is each item in the Vec.
            .map(|field| field.trim())
            // Build a collection of fields
            .collect();

        // Checked the configuration at compile time
        if cfg!(debug_assertions) {
            // Print to stderr. These will be omitted when the code is compiled in release mode (cargo run --release or cargo run --release -q). This conditional is provided by the cfg! macro.
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        // Attempt to parse the second item in field as a float, if successful, then assign the value to the length variable
        // "if let" is a concise method of conditionally processing data that also provides a local variable assigned to the that data.
        // The "parse" method returns Ok(T) (where T stands for any type) when it can successfully parse the string. Otherwise is returns Err(E) where E is for an Error type.
        // The use of "if let" here means it'll skip over any lines that can't be parsed as floats, ie the "Invalid,data" line.
        if let Ok(length) = fields[1].parse::<f32>() {
            // Print to stdout
            println!("{}, {}cm", name, length);
        }
    }
}