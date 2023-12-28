fn main() {
    more_verbose();
    less_verbose_if_let();
    if_let_with_else();
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
fn more_verbose() {
    // This is more verbose because we have to satisfy the requirements of the match pattern by including the default, catch all arm.
    // An alternative would be to use it let (shown in less_verbose_if_let())
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}

fn less_verbose_if_let() {
    // If let is syntax sugar that will run if the value matches ONE pattern and ignores all other values
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn if_let_with_else() {
    count_coins_with_default(CoinWithValue::Quarter(UsState::Alabama)); // State quarter from Alabama
    count_coins_with_default(CoinWithValue::Nickel);                    // Coin is not a quarter!
    count_coins_with_default(CoinWithValue::Dime);                      // Coin is not a quarter!

    count_coins_with_if_let_else(CoinWithValue::Quarter(UsState::Alabama)); // [if_let_else] State quarter from Alabama
    count_coins_with_if_let_else(CoinWithValue::Nickel);                    // [if_let_else] Coin is not a quarter!
    count_coins_with_if_let_else(CoinWithValue::Dime);                      // [if_let_else] Coin is not a quarter!
}

fn count_coins_with_default(coin: CoinWithValue) {
    let mut count = 0;
    match coin {
        CoinWithValue::Quarter(state) => println!("State quarter from {:?}", state),
        _ => {
            count += 1;
            println!("Coin is not a quarter!");
        },
    }
}

fn count_coins_with_if_let_else(coin: CoinWithValue) {
    let mut count = 0;
    if let CoinWithValue::Quarter(state) = coin {
        println!("[if_let_else] State quarter from {:?}", state);
    } else {
        count += 1;
        println!("[if_let_else] Coin is not a quarter!");
    }
}



