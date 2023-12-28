fn main() {
    create_user_struct_instance();
    create_user_struct_instance_shorthand(String::from("shorthand_username"), String::from("shorthand_email@c.c"));
    create_user_from_another_user();
    create_user_using_struct_update_syntax();
    tuple_structs();
}

struct User {
    // Note: It's possible to use references for the types here, but to do so, you'd need to specify "lifetimes" (which is explained in later chapters).
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn create_user_struct_instance() -> User {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someemail123@c.com"),
        sign_in_count: 1,
    };

    println!("Users email: {}", user1.email);
    user1.email = String::from("newemail@c.c");
    println!("Users new email: {}", user1.email);

    user1
}

fn create_user_struct_instance_shorthand(username: String, email: String) {
    // This works because the parameters in the function signature have the SAME NAME as the User struct fields.
    let user1 = User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    };

    println!("Shorthand user email: {}", user1.email);
}

fn create_user_from_another_user() {
    let user1 = create_user_struct_instance();

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("anotheruser2@c.c"),
        sign_in_count: user1.sign_in_count,
    };

    println!("User2 created by coping most of user1 values: {}", user2.email);
}

fn create_user_using_struct_update_syntax() {
    let user1 = create_user_struct_instance();

    let user2 = User {
        email: String::from("anotheruser2@c.c"),
        // This MUST come last!
        ..user1
    };

    // The struct update syntax MOVES the data from user1 to user2. So we can't use the data from user1. This will error at compile time:
    //println!("{}", user1.username);

    // However, because we didn't move the email field from user1, this will work:
    println!("User1 email: {}", user1.email);

    // Note that because the active and sign_in_count properties are types that implement the Copy trait (ie, they are copied instead of moved to user2), this will work:
    println!("User1 sign_in_count: {}", user1.sign_in_count);

    println!("User2 created with struct update syntax: {}", user2.email);
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() {
    // This are basically named tuples. Useful for when you don't need the extra verbiage that comes with a struct, eg color codes or coordinates:
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black: {}, {}, {}", black.0, black.1, black.2);
    println!("Origin: {}, {}, {}", origin.0, origin.1, origin.2);
}


struct AlwaysEqual;

fn unit_like_structs() {
    let subject = AlwaysEqual;

}