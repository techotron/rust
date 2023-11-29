fn main() {
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2)); // Numbers have methods. This is 1_000_000^2

    // This creates an array of floating point numbers which are all 42.0. The numbers must be the same type
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32
    ];

    println!("{:02}", forty_twos[0]);
}