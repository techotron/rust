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
}
