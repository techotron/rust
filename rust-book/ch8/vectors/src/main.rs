fn main() {
    create_empty_vector();
    create_vector_with_macro();
    update_vector();
    reading_vector_values();
    iterate_over_vector();
    vector_with_different_types();
}

fn create_empty_vector() {
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v)
}

fn create_vector_with_macro() {
    let v = vec![1, 2, 3];
    println!("{:?}", v)
}

fn update_vector() {
    let mut v = Vec::new();
    v.push(4);
    v.push(5);
    v.push(6);
    println!("{:?}", v)
}

fn reading_vector_values() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("[using index] The third element is {third}");

    let x = vec![1, 2];
    let third: Option<&i32> = x.get(2);
    match third {
        Some(third) => println!("[using get + match] The third element is {third}"),
        None => println!("[using get + match] There is no third element."),
    }
}

fn iterate_over_vector() {
    let v = vec![1, 2, 3, 4, 5, 6];
    for i in v {
        println!("v: {i}");
    }

    let mut mut_v = vec![7, 8, 9, 10];
    for i in &mut mut_v {
        *i += 50;
        println!("mut_v: {i}")
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vector_with_different_types() {
    // Vectors can only contain elements of the same type. Enums count as the same type so this works:
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row)
}