/*
    A trait defines functionality a particular type has and can share with other types. 
    We can use traits to define shared behavior in an abstract way. 
    We can use trait bounds to specify that a generic type can be any type that has certain behavior.

    Note: Traits are similar to a feature often called interfaces in other languages, 
    although with some differences.
*/

pub trait Summary {
    // Each type implementing this trait must provide its own custom behavior 
    fn summarize(&self) -> String;
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
/*
    Implementing a trait on a type is similar to implementing regular methods. The difference is that after impl, we put the trait name we want to implement, then use the for keyword, and then specify the name of the type we want to implement the trait for
 */
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    println!("Hello, world!");
}
