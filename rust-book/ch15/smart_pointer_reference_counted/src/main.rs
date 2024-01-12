// A Reference Counted smart pointer is one which keeps track of how many references
//  exist to some data. The data won't be cleaned up until there are zero references
//  to it.

// The data structure below can be visualised like this:

//  b -> (3) --|
//             |
//             `--- a -> 5 --> 10 --> Nil
//             |
//  c -> (4) --|

// b and c are separate lists which both share list a
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
// The Rc<T> isn't in the prelude so we need to bring it into scope
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a: {}", Rc::strong_count(&a));
    // Rc::clone doesn't make a deep copy of all the data like most types do.
    //  The call to Rc::clone only increments the reference counter to the data which
    //  isn't expensive (in performance terms). Most other implementations of clone()
    //  create deep copies which can be costly.
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b: {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c: {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope: {}", Rc::strong_count(&a));
}
