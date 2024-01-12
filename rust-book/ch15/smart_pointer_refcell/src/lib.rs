// A RefCell allows an immutable value to be mutated. The compiler relies on the code
//  to implement the ownership rules are adhered to. The compiler's borrow checker will
//  will allow this "interior mutability" but if the borrowing rules are checked at
//  runtime instead and will panic if the rules are violated.

// An example, of the ownership rule which needs to be applied is this: it won't compile
//  because we're trying to borrow y as a mutable reference from the immutable x.
// fn main() {
//     let x = 5;
//     let y = &mut x;
// }


// This trait is the interface that a mock object can implement so the mock object can
//  be used in the same way a real object is.
pub trait Messenger {
    // Note that the send method here takes an immutable self reference value
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    // The MockMessenger struct is to keep track of the sent_messages which are sent.
    struct MockMessenger {
        // Here, we wrap the sent_messages value into a RefCell, which allows us to
        //  mutate self in our implementation of the send() method. Without doing this
        //  the compiler would complain that we were trying to "push()" a string into
        //  an immutable self value inside the send() method.
        sent_messages: RefCell<Vec<String>>,
    }

    // The new() function makes it convenient to create new MockMessenger values which
    //  start with an empty list of values.
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    // We need to implement the Messenger trait so we can give a MockMessenger type to
    //  a LimitTracker.
    impl Messenger for MockMessenger {
        // To implement the Messenger trait, we need to create a send method. In this
        //  implementation, we'll capture the message being sent and place it into a
        //  vector.
        fn send(&self, message: &str) {
            // Because self.sent_messages is a RefCell, we can call the borrow_mut()
            //  method to get a mutable reference to the value inside which allows us
            //  to call push().
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        // Create a new limit_tracker and give it a max of 100
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        // Set the current "use" value to 80 (more than 75% of 100).
        limit_tracker.set_value(80);

        // Because mock.messenger.sent_messages is a RefCell type, we have to call
        //  borrow() to get an immutable reference to the value it wraps (which we
        //  can then call len() on).
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}