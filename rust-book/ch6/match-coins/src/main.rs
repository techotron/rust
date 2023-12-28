fn main() {
    match_example();
    match_example_with_enum_value(CoinWithValue::Quarter(UsState::Alabama));
    match_with_option_example();
    roll_dice();
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn match_example() {
    println!("A penny is worth: {}", value_in_cents(Coin::Penny));
    println!("A dime is worth: {}", value_in_cents(Coin::Dime));
}


#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
//     ...
}

enum CoinWithValue {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn match_example_with_enum_value(coin: CoinWithValue) -> u8 {
    match coin {
        CoinWithValue::Penny => 1,
        CoinWithValue::Nickel => 5,
        CoinWithValue::Dime => 10,
        CoinWithValue::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn match_with_option_example() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => {
            println!("x is None");
            None
        }
        Some(i) => {
            // Some(i) binds the value of x to the variable, "i", meaning we can using it in this arm
            println!("x is not None: {}", i);
            Some(i + 1)
        }
    }
}


fn roll_dice() {
    match_with_default_pattern(7);                  // Expected: Removing a fancy hat!
    match_with_default_pattern(8);                  // Expected: Moving the player 8 spaces!
    match_with_default_pattern(3);                  // Expected: Adding a fancy hat!
    match_where_nothing_happens_in_default_arm(7);  // Expected: Removing a fancy hat!
    match_where_nothing_happens_in_default_arm(1);  // Expected: (nothing)
}

fn match_with_default_pattern(dice_roll: i32) {
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // The variable name here could be anything, doesn't have to be "other". If we didn't want to use the value caught in this catch-all arm, we could use the and underscore instead of "other".
        other => move_player(other),
    }
}

fn add_fancy_hat() {
    println!("Adding a fancy hat!")
}

fn remove_fancy_hat() {
    println!("Removing a fancy hat!")
}

fn move_player(moves: i32) {
    println!("Moving the player {} spaces!", moves)
}


fn match_where_nothing_happens_in_default_arm(dice_roll: i32) {
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => ()
    }
}