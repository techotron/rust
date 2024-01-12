use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

// Even though hello() takes in a &str parameter, we can hand in MyBox. This works
//  because MyBox implements the Deref trait, so Rust can turn a &MyBox<String> into
//  &String. This is called "deref coercion".
// If Rust didn't have this feature, we would have to deref MyBox and change it to a
//  reference slice ourselves, like this:
// hello(&(*m)[..]);
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
