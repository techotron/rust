fn main() {
    matching_literals();
    matching_named_variables();
    matching_ranges();
    destructuring_structs();
    destructuring_enums();
    destructuring_nested_structs_and_enums();
    destructuring_combos();
    match_guard();
    at_bindings();
}

// Useful for when you want the code to take an action, depending whether it gets a particular
//  concrete value
fn matching_literals() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

// This is an example of how the variables in the scope of the match arms, can shadow the outer
//  scope's variables.
fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // Some(y) here is capturing the "Some" value from x (in this case, 4) and binding it to
        //  the variable, y. We've already defined a variable y with a value of 9. This code shadows
        //  that variable inside the scope of the match block. When the code exists the match scope,
        //  the original (unshadowed) value of y is printed.
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    // Output:
    // Matched, y = 5
    // at the end: x = Some(5), y = 10
}

fn matching_ranges() {
    let x = 5;

    match x {
        // This pattern matches i32's, 1 - 5 (inclusive).
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // Similar example but with chars instead of i32
    let y = 'c';

    match y {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

fn destructuring_structs() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    // This line generates the variables a and b to equal the values of p.x and p.y respectively.
    //  However, if you were to name the variables the same as the struct fields, then you can use
    //  the shorthand syntax below...
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // Shorthand syntax
    let Point { x, y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // We can also destructure with literal values as part of the struct pattern. This allows us to
    //  test some of the fields for particular values.
    // Note: the match expression stops at the first matching pattern, so even if x and y were both 0,
    //  it would stop and the first arm.
    match p {
        Point { x, y: 0} => println!("y is zero. On the x axis at: {x}"),
        Point { x: 0, y} => println!("x is zero. On the y axis at: {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}


fn destructuring_enums() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no destructure");
        }
        Message::Move { x, y } => {
            println!("The destructure pattern is similar to that of a struct");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red: {r}, green: {g}, blue: {b}");
        }
    }
}

fn destructuring_nested_structs_and_enums() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue: {h}, saturation: {s}, value: {v}");
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red: {r}, green: {g}, blue: {b}");
        }
        _ => (),
    }

}


fn destructuring_combos() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("Feet: {feet}, Inches: {inches}, x: {x} y: {y}");
}


// Match guards contain additional if statements to include more complex logic before an arm is chosen
// The downside to this approach is that the compiler doesn't check if all the arms and guards are
//  exhaustive.
fn match_guard() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    // We can also use match guards to solve the variable shadowing problem inside matching_named_variables()
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    // Output:
    // Default case, x = Some(5)
    // at the end: x = Some(5), y = 10

    let a = 4;
    let b = false;

    match a {
        4 | 5 | 6 if b => println!("yes"),
        _ => println!("no"),
    }

    // Output:
    // no
}

// We can bind the variable found in a match arm using the @ key
fn at_bindings() {
    enum Message {
        Hello { id: i32 }
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        // This binds the new var: id_variable if the id value in the Message::Hello is within the
        //  given range. In this
        Message::Hello { id: id_variable @ 3..=7,} => println!("Found id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id {id}"),
    }
}