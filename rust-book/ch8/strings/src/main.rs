fn main() {
    strings_and_string_literals();
    concatenate_strings();
    string_indexing();
    iterating_over_strings();
}

fn strings_and_string_literals() {
    // str (aka "string slice" or just "slice") is a reference to some UTF-8 data stored on the heap somewhere.
    //  As it's accessed behind a pointer, it's usually seen as "&str".
    //  It is immutable

    // String is a dynamic string type. It is mutable, growable, owned UTF-8 encoded.

    //////////////////

    let string_literal = "initial contents";

    let string_type = string_literal.to_string();
    // let string_type = "initial contents".to_string();    // also valid
    // let string_type = String::from("initial contents");  // same thing

    println!("{string_type}")
}

fn concatenate_strings() {
    // push_str ==================================
    let mut foo = String::from("foo");
    let bar = String::from("bar");
    println!("{foo}");
    foo.push_str(&bar);
    println!("{foo}");
    // Can still use bar here because we didn't transfer ownership in the push_str method
    println!("{bar}");

    // push ======================================
    let mut lol = String::from("lo");
    lol.push('l');
    // Single quotes for characters and double quotes for strings:
    //  lol.push("l");      // This would fail
    //  lol.push('lly');    // This would fail
    println!("{lol}");

    // plus operator ==============================
    let hello = String::from("hello, ");
    let world = String::from("world!");
    let hello_world = hello + &world;
    println!("{hello_world}");
    println!("{world}");
    // println!("{hello}"); // This would fail because hello has been moved and can no longer be used
    //  The plus operator uses the "add" method, whose signature looks like this:
    //  fn add(self, s: &str) -> String {...
    //  In this case, we're calling the add method on the hello String, so we're passing in world as the &str into the add method.
    //  However, &world is a &String, not a &str - this still works because Rust is using "deref coercion" to turn &world into &world[..] (aka, a &str)
    //
    //  add takes ownership of self (in this case, our "hello" variable). As it's moved into the add method, we can't use it after the concatenation operation

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    // You can add multiple references to append but this isn't scalable
    let tictactoe = tic + "-" + &tac + "-" + &toe;
    println!("{tictactoe}");

    // Using the format! macro =======================
    let tic = String::from("tic");  // Need to recreate because it was moved in the previous operation
    let format_tictactoe = format!("{tic}-{tac}-{toe}");
    println!("{format_tictactoe}");
}

fn string_indexing() {
    // tldr; You can't.

    // This will fail:
    // let s = String::from("hello");
    // println!("{}", s[0]);

    // It fails because a String is a wrapper over a Vec<u8>. Some UTF-8 encoded characters can take up 2 bytes of storage,
    //  so the index of an element in the String, doesn't necessarily correlate to the position it takes in the vector.
}

fn iterating_over_strings() {
    let hello = String::from("hello");
    for c in hello.chars() {
        println!("{c}");
    }

    // Even though there are only 2 characters in the string literal, each char takes up 2 bytes of storage so it'll print x4 numbers
    let two_byte_unicodes = String::from("Зд");
    for c in two_byte_unicodes.bytes() {
        println!("{c}");
    }

    // Calling chars() will only print the number of characters in the string
    for c in two_byte_unicodes.chars() {
        println!("{c}");
    }
}