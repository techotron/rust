// This is a demo function which can apply to multiple sources of information. It aims
// to summarise headlines from news articles and tweets etc.

// Anything which implements the Summary trait will need to have a function which
//  matches the function signatures defined within it (unless a default is implemented)
pub trait Summary {
    // By default, this will return a string literal. The default would apply if
    //  the method is missing for a struct which implements the Summary trait.
    fn summarize(&self) -> String {
        String::from("Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// This is how we implement a trait on a method "impl TRAIT_NAME for STRUCT_NAME"
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by, {} ({})", self.headline, self.author, self.location)
    }
}

pub struct MissingSummarize {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Create an empty implementation so that the trait is used but allows the default
//  "summarize()" method to get used from the trait.
impl Summary for MissingSummarize {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// This function takes in the Summary trait as a parameter and calls the summarize method
//  from it.
//  A longer syntax for this signature is called "a trait bound" and would be expressed by
// pub fn notify<T: Summary>(item: &T) {
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// We can also specify a function to implement multiple traits:
// As trait bound syntax, this would be:
// pub fn notify2<T: Summary + Display>(item: &T) {
pub fn notify2(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds. some_function1 and some_function2 specify that the first
//  generic (T) will inherit the Display and Clone traits, whilst the second generic
//  (U) will inherit the Clone and Debug traits
fn some_function1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {1}

// Adding multiple trait bounds to a function can become clumsy to read so Rust has the
//  where clause to make it more legible:
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{1}

// You can return a type that implements a trait in the return position of the function
//  We are specifying that some type that implements the Summary trait is returned, rather
//  than the concrete type itself.
// This only works if you are returning a single type. If this function returned either a Tweet
//  or a NewsArticle, it wouldn't work, even though both implement the Summary trait. This
//  is possible but is covered in a later chapter.
fn returns_summarizable() -> impl Summary {
   Tweet {
       username: String::from("horse_ebooks"),
       content: String::from(
           "of course, as you probably already know, people",
       ),
       reply: false,
       retweet: false,
   }
}

fn main() {
    // NOTE: if this was a library, then the users would need to bring the trait into
    //  scope, as well as the types, Tweet and/or NewsArticle
   let tweet = Tweet {
       username: String::from("horse_ebooks"),
       content: String::from(
           "of course, as you probably already know, people",
       ),
       reply: false,
       retweet: false,
   };

   println!("1 new tweet: {}", tweet.summarize());

    let news1 = MissingSummarize {
        headline: String::from("some headline"),
        location: String::from("Wirral"),
        author: String::from("snow"),
        content: String::from("some new content"),
    };

    println!("New article available: {}", news1.summarize());

    // file:///Users/edward.snow/git/github.com/rust-lang/book/book/ch10-02-traits.html#trait-bound-syntax
}
