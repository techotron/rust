#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods use the impl keyword (implementation). Everything within this block will be associated with the Rectangle type. The main reason for this is to group all the related capabilities for a Rectangle into one impl block.
// All functions inside the impl block are called "associated functions".
impl Rectangle {
    // The &self parameter is short for self: &Self. This is an alias for the type and the impl block is for. Note that we still need to notate it with & to specify that we're borrowing the Self instance. We could take ownership of it here but we don't want to.
    // If we wanted to edit the values inside of self, we would set the function signature was &mut self as the first parameter.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // We can define a method which is named the same as one of the structs fields. We would call this method by using parentheses
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        self.width > rect2.width && self.height > rect2.height
    }

    // This is an associated function which doesn't have self as it's first parameter. These are known as constructors because they don't need an instance of the type to work with. String::from is an example of such a constructor.
    // Constructors are often called "new" but this is a convention rather than a rule.
    fn square(size: u32) -> Self {
        // The "Self" keyword here is an alias for the type that the impl block is associated with. In this case, calling "Rectangle::square(3)" would return a Rectangle type where the width and height are 3.
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Width is greater than 0: {}", rect1.width());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::square(3);
    println!("My new square is: {:?}", square1)
}
