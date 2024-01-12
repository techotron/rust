// This example shows how instead of using Rc::clone() (which increments the strong_count
//  value), we can use Rc::upgrade/downgrade (which increments/decrements the weak_count
//  value). Unlike the strong_count value, the weak_count doesn't have to be zero before
//  the data is dropped from the heap.
// When Rc::upgrade is called, it'll return an Option<Rc<T>>. We'll get a result of Some
//  if the Rc<T> value hasn't been dropped yet, or a None if it has - meaning we won't
//  get an invalid pointer value if the underlying data has been dropped.

// This example is a tree, whose items know about their children items and their parent
//  items.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

// We want a node to own it's children and we want to share that ownership with variables
//  so we can access each Node in the tree directly. So we do this by setting children to
//  be a Vec of reference counted smart pointers to a Node type. We also want to modify
//  which nodes are children of other nodes so we wrap that all in a RefCell<T>.
// The parent field uses a Weak ref counted smart pointer. We want child nodes to be aware
//  of their parents. If we used a Rc for parent, then a child and parent relationship would
//  always be at least 1 (because it would create a circular reference). We want a child node
//  to be dropped, if the parent node is dropped however, we don't want a parent node to be
//  dropped if the child node is also dropped.
// The weak ref here will allow an instance of Node to be dropped, even if the weak ref count
//  is not 0.
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // leaf has no children (empty vector) and a value of 3
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // This returns None - because the leaf.parent doesn't exist
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // branch has one child (a vec with a Rc reference to leaf) and a value of 5
    // Now, the node in leaf has 2 owners ("leaf" itself and "branch")
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // Now we have the leaf node instance in branch.children, we can modify leaf to
    //  give it a parent by returning a Weak<Node> reference to branch by calling
    //  Rc::downgrade(&branch)
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}