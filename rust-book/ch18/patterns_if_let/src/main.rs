fn main() {
    // let favorite_color: Option<&str> = Some("red");
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
        // if let will bind the value to the variable inside the Type, eg, here, we're binding the age
        //  from Ok(age) to be the value of age. Because age is already a defined variable from line 5
        //  and we're re-assigning it, it's called a shadow variable.
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
