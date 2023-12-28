#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of rect1 is {} square pixels", area(&rect1));

    area_with_debug_trait(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    // We use an immutable reference in the function parameter because we only want to borrow the values of rect, NOT take ownership of it.
    rectangle.width * rectangle.height
}


fn area_with_debug_trait(rectangle: &Rectangle) -> u32 {
    // We can use the println macro to format our struct because we annotated the struct with #[derive(Debug]
    println!("{:?}", rectangle);
    println!("{:#?}", rectangle);
    rectangle.width * rectangle.height
}
