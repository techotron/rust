// A state pattern is an object oriented design pattern. The state "value" defines the behaviour of
//  the object.

// In this example, we have a blog post which will only display content when it's in an approved
//  state.
use state_pattern::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
