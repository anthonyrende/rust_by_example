/*
    A trait defines functionality a particular type has and can share with other types. 
    We can use traits to define shared behavior in an abstract way. 
    We can use trait bounds to specify that a generic type can be any type that has certain behavior.

    Note: Traits are similar to a feature often called interfaces in other languages, 
    although with some differences.
*/

// pub trait Summary {
//     // Each type implementing this trait must provide its own custom behavior 
//     fn summarize(&self) -> String;
// }

use::traits::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("new_user"),
        content: String::from("content herer"),
        reply: false,
        retweet: true,
    };

    println!("1 New tweet {:#?}", tweet);
}
