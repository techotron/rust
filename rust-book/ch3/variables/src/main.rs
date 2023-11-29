fn main() {
    let immutable_variable = 5;
    println!("The value of immutable_variable is {immutable_variable}");

    let mut mutable_variable = 5;
    println!("The value of x is {mutable_variable}");
    mutable_variable = 6;
    println!("The value of x is {mutable_variable}");

    // Shadowing
    let shadow_variable = 5;
    // note the use of let in the shadow variable here. This is effectively creating a new variable (rather than mutating the existing one)
    let shadow_variable = shadow_variable + 1;

    {
        // at this point, shadow_variable is 6
        let shadow_variable = shadow_variable * 2;
        // in this scope, shadow_variable is 12
        println!("The value of shadow_variable is {shadow_variable}");
    }

    // at this point, shadow_variable is 6
    println!("The value of shadow_variable is {shadow_variable}");

    // shadowing allows us to change the type of the variable
    let spaces = "   ";
    println!("The value of spaces is '{spaces}'");
    let spaces = spaces.len();
    println!("The length of spaces is {spaces}");

    let my_bool: bool = true;
    println!("my_bool is '{my_bool}'");

    // Compound Types (grouped values): Tuples and Arrays
    // Tuples can contain values of different types
    let my_tuple: (i32, f64, u8) = (500, 6.4, 1);

    // This is called "destructuring"
    let (x, y, z) = my_tuple;
    println!("my_tuple (x) is '{x}'");
    println!("my_tuple (y) is '{y}'");
    println!("my_tuple (z) is '{z}'");

    // We can also access elements within a tuple, using the index:
    let my_tuple_idx_one = my_tuple.1;
    println!("my_tuple at index 1 is: '{my_tuple_idx_one}'");

    // Arrays must contains values of the same type and have a fixed length (unlike a vector type which can grow/shrink)
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    let my_array_same_value: [&str; 5] = ["my_value"; 5];

    for i in &my_array {
        println!("my_array is: '{i}'");
    }

    for i in 0..my_array_same_value.len() {
        let my_temp_value = my_array_same_value[i];
        println!("my_array_same_value: '{my_temp_value}'");
    }
}
