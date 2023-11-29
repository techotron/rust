use std::collections::HashMap;

fn main() {
    println!("======= get_fib()");
    let get_fib_result = get_fib(11); // should be 144
    println!("get_fib = {get_fib_result}");

    println!("======= twelve_days_of_christmas()");
    twelve_days_of_christmas();
}


fn get_fib(number: i32) -> i32 {
    let mut fib_zero = 0;
    let mut fib_one = 1;

    let mut result = 0;

    for _item in 0..number {
        result = fib_zero + fib_one;
        fib_zero = fib_one;
        fib_one = result;
    }
    return result
}


fn twelve_days_of_christmas() {

    let mut daily_lyrics = HashMap::new();
    daily_lyrics.insert(0, ("first", "A partridge in a pear tree"));
    daily_lyrics.insert(1, ("second", "Two turtle doves"));
    daily_lyrics.insert(2, ("third", "Three french hens"));
    daily_lyrics.insert(3, ("forth", "Four calling birds"));
    daily_lyrics.insert(4, ("fifth", "Five gold rings"));
    daily_lyrics.insert(5, ("sixth", "Six, geese a-laying"));
    daily_lyrics.insert(6, ("seventh", "Seven swans a-swimming"));
    daily_lyrics.insert(7, ("eighth", "Eight maids a-milking"));
    daily_lyrics.insert(8, ("ninth", "Nine ladies dancing"));
    daily_lyrics.insert(9, ("tenth", "Ten lords a-leaping"));
    daily_lyrics.insert(10, ("eleventh", "Eleven pipers piping"));
    daily_lyrics.insert(11, ("twelfth", "Twelve drummers drumming"));


    for day in 0..12 {
        let (day_word, gift) = daily_lyrics[&day];
        println!("For the {day_word} day of christmas, my true love sent to me,\n{gift},");
        for other_day in (0..day).rev() {
            let (_, other_gift) = daily_lyrics[&other_day];
            println!("{other_gift},")
        }
    }
}