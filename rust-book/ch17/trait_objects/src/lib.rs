// We want to create a library which can be used by a GUI. The draw method will be used to display an
//  image of a button/gui element on the screen. We don't know the exact type of the element though, so
//  to allow for this ambiguity in the library, we will use a trait object called "Draw" which defines
//  a draw() method.
pub trait Draw {
    fn draw(&self);
}

// We want to define a vector which takes a trait object which we can call draw() on each. This is done
//  using the below syntax, as used for "components".
// Box<dyn Draw> is a stand-in for any type inside a Box, that implements the Draw trait.
// This is different from using a generic type parameter, because a generic type parameter can only be
//  substituted with one concrete type at a time. Using trait objects however, we can use multiple concrete
//  types at a time.
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Button type will implement the Draw trait so it can be used inside Screen::components
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("drawing a button here: {}", &self.label)
    }
}
