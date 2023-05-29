//  Bringing HashMap into scope in an idiomatic way
use std::collections::HashMap;
// The exception to this idiom is if we’re bringing two items with the same name into scope with use
use std::fmt;
// use std::io;
use std::io::Result as IoResult;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// We define a module with the mod keyword followed by the name of the module
mod front_of_hose {
    // . Inside modules, we can place other modules, as in this case with the modules hosting and serving
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// e can create a shortcut to a path with the use keyword once, and then use the shorter name everywhere else in the scope.
use crate::front_of_hose::hosting;

pub fn eat_at_restaurant() {
    // using the shortcut
    hosting::add_to_waitlist();
    // Absolute path
    crate::front_of_hose::hosting::add_to_waitlist();

    // Relative path
    front_of_hose::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread to order
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // In contrast, if we make an enum public, all of its variants are then public
}

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        //  we can use super to go to the parent module of back_of_house (which is crate the root in his case) then to deliver order
        super::deliver_order();
    }

    fn cook_order() {}
}
