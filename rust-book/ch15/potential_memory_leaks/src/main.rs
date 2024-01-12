use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    // Create a, which is a new Rc of List
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));              // 1
    println!("a next item = {:?}", a.tail());                               // Some(RefCell { value: Nil})

    // Create b, which is a new Rc of List which references a
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));     // 2
    println!("b initial rc count = {}", Rc::strong_count(&b));              // 1
    println!("b next item = {:?}", b.tail());                               // Some(RefCell { value: Cons(5, RefCell { value: Nil }) })

    // a.tail() will match "self" (which is a Cons enum), where it'll return a
    //  Some(RefCell { value: Nil }) and bind it to "link".
    if let Some(link) = a.tail() {
        // Link is a RefCell smart pointer, so we can call "borrow_mut", giving us a
        //  mutable reference to it, allowing us to set the value to a reference of b.
        *link.borrow_mut() = Rc::clone(&b);
    }

    // b now has 2 references counted because now it's also used in a.
    println!("b rc count after changing a = {}", Rc::strong_count(&b));     // 2
    println!("a rc count after changing a = {}", Rc::strong_count(&a));     // 2

    // b was created with a reference to a
    // a was mutated to have a reference to b
    // We now have a circular reference - aka a memory leak.

    // If we uncomment this line, the execution will exit with a stack overflow error,
    //  because a.tail() returns Some(..b) which is Some(..a) which is Some(..b) ...etc
    // println!("a next item = {:?}", a.tail());
}
