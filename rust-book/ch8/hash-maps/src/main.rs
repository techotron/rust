use std::collections::HashMap;

fn main() {
    simple_hash_map();
    hash_map_get_method();
    loop_over_hash_map();
    hash_map_ownership_rules();
    overwriting_hash_map_value();
    adding_key_value_if_key_is_missing();
    updating_the_value_based_on_old_value();
}

fn create_new_hash_map() -> HashMap<String, i32> {
    // Note: no built-in macro to create a new hash map (because it's the least used out of the 3 common collections.
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    scores

    // HashMaps store their data on the heap (like vectors).
    //  - All keys must be of the same type
    //  - All values must be of the same type
}

fn simple_hash_map() {
    let scores = create_new_hash_map();
    println!("{:?}", scores);
}

fn hash_map_get_method() {
    let scores = create_new_hash_map();

    let team_name = String::from("blue");
    // If get() doesn't find a key, it'll return None. This program handles the Option by calling
    //  copied() to get an Option<i32> instead of Option<&i32>. Then unwrap_or() to set score to 0,
    //  if the Option result is None.
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue's score is {score}");
}

fn loop_over_hash_map() {
    let mut scores = create_new_hash_map();
    scores.insert(String::from("green"), 70);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn hash_map_ownership_rules() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, so this would fail:
    //println!("{field_name}");
}

fn overwriting_hash_map_value() {
    let mut scores = create_new_hash_map();
    println!("[overwriting_hash_map_value] Score for blue: {:?}", scores.get("blue").copied().unwrap_or(0));
    scores.insert(String::from("blue"), 20);
    println!("[overwriting_hash_map_value] New score for blue: {:?}", scores.get("blue").copied().unwrap_or(0));
}

fn adding_key_value_if_key_is_missing() {
    let mut scores = create_new_hash_map();

    // The return key from the entry() method is an Entry, that represents a value that might or might not exist.
    // The or_insert() method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists.
    // If the key doesn't exist, it's created with the value in the parameter.
    scores.entry(String::from("green")).or_insert(50);
    scores.entry(String::from("blue")).or_insert(50);

    println!("Scores: {:?}", scores);
}

fn updating_the_value_based_on_old_value() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // This is altering the value of the dereferenced "count" variable which changes the value for the key.
        //  This works because the return value of or_insert() is a mutable reference (&mut V).
        *count += 1;
    }

    println!("{:?}", map);
}