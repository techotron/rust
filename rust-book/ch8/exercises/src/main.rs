use std::collections::HashMap;

fn main() {
    let mut even_int_vector: Vec<i32> = vec![12, 45, 1, 65, 3, 95, 31, 32, 95, 75, 12, 81];
    let mut odd_int_vector: Vec<i32> = vec![12, 45, 1, 65, 3, 95, 31, 32, 95, 75, 12, 81, 43];
    let sentence = "This is something which should be changed into piglatin Apple";

    find_median(&mut even_int_vector);  // Should equal 38.5
    find_median(&mut odd_int_vector);   // Should equal 43
    find_mode(&mut even_int_vector);    // Should equal 12, 95
    convert_to_pig_latin(&sentence);
}

// Finds the mean of a given list of integers
fn find_mean(numbers: Vec<i32>) -> f64 {
    let mut mean: i32 = 0;
    let length = numbers.len();
    for i in numbers {
        mean = mean + i;
    }

    mean as f64 / length as f64
}

// Finds the median from a list of integers
fn find_median(numbers: &mut Vec<i32>) {
    numbers.sort();
    if (numbers.len() % 2) > 0 {
        // The length of the vec is odd. Need to find the middle number (note, this is not the same as divide by 2)
        //  We add 1 to the length, divide by 2 and minus 1 to get the index for the middle of the vec
        let median_idx = (numbers.len() + 1) / 2;
        println!("Odd median is {}", numbers[median_idx -1])
    } else {
        // The length of the vec is even. Find the middle 2 numbers and calculate the mean
        let median_idx = numbers.len() / 2;
        let mean_vec = vec![numbers[median_idx -1], numbers[median_idx]];
        println!("Even median is {:?}", find_mean(mean_vec))
    }
}

// Finds the mode (number which occurs the most)
fn find_mode(numbers: &mut Vec<i32>) {
    let mut numbers_hash = HashMap::new();

    let mut max = 0;
    for i in numbers {
        let count = numbers_hash.entry(i).or_insert(0);
        *count += 1;
        if count >= &mut max {
            max = *count;
        }
    }

    let mut mode = vec![];
    for (key, value) in numbers_hash {
        if value == max {
            mode.push(key);
        }
    }
    println!("{:?}", mode)
}

// Convert strings to pig-latin
fn convert_to_pig_latin(sentence: &str) {
    let mut pig_sentence: Vec<String> = vec![];
    let vowels: Vec<&str> = vec!["a", "e", "i", "o", "u"];
    for word in sentence.split_whitespace() {
        if vowels.iter().any(|&v| word.to_lowercase().starts_with(v)) {
            let pig_word = format!("{word}-hay ");
            pig_sentence.push(pig_word);
        } else {
            // nth() is an iterator method which gets the Nth element. It returns either T or None
            // unwrap() will return T or panic if None. It's fine to use here but ideally should be handled better
            let first_letter = word.chars().nth(0).unwrap();
            let pig_word = format!("{}-{}ay ", remove_first_character(word), first_letter);
            pig_sentence.push(pig_word);
        }
    }

    let result = String::from_iter(pig_sentence);
    println!("{result}")
}


fn remove_first_character(word: &str) -> &str {
    let mut chars = word.chars();
    chars.next();
    chars.as_str()
}