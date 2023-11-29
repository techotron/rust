fn main() {

    // Infinite loop
    // loop {
    //     println!("again!");
    // }

    println!("======== Loop with counter:");
    loop_with_counter();

    println!("======== Using loop labels:");
    labelled_loops();

    println!("======== While loop:");
    while_loop();

    println!("======== Loop over arrays");
    array_loop();
    more_concise_array_loop();

    println!("======== Loop over range");
    range_loop();
}

fn loop_with_counter() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}


fn labelled_loops() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn array_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

fn more_concise_array_loop() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("[more concise] the value is {element}")
    }
}

fn range_loop() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}