// Here, we build our own smart pointer, to demonstrate how they differ from a
//  reference.
use std::ops::Deref;


// Define a new struct and declare a generic parameter, T.
//  This is a tuple struct, meaning a struct with unnamed fields.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

struct MyBox2<T>(T);

impl<T> MyBox2<T> {
    fn new(x: T) -> MyBox2<T> {
        MyBox2(x)
    }
}

impl<T> Deref for MyBox2<T> {
    // This syntax defines an associated type for the Deref trait to use.
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // This returns the reference to the first value from a tuple struct.
        &self.0
    }
}

fn main() {
    let x = 5;
    let _y = MyBox::new(x);
    let z = MyBox2::new(x);

    assert_eq!(5, x);
    // This will fail because MyBox doesn't implement the "Deref" trait.
    //assert_eq!(5, *_y);

    // Because MyBox2 implements the necessary methods for the Deref trait, we can
    //  use the deref operator on it and the assertion will pass.
    // Behind the scenes, when we call *z, Rust is really running: *(z.deref())
    //  The deref outside the parenthesis is to make sure that if deref() returned the
    //  actual value (instead of a reference) then we'd transfer ownership of the value
    //  to MyBox2 (which we don't want). By using a deref operator here, we're ensuring
    //  we don't transfer ownership.
    assert_eq!(5, *z);
}
