use rand::Rng;
use std::cmp;
use std::io;

fn main() {
    loop {
        println!("Guess the number!");
        println!("Please input your guess.");

        let secret_number = rand::thread_rng().gen_range(1..=10);

        let mut guess = String::new();

        // std::io::Stdin a type that represents a handle to the standard input for your terminal.
        io::stdin()
            // read_line() takes whatever the user types into standard input and places that into a string, so it takes that string as an argument.
            .read_line(&mut guess)
            // read_line() returns a value of io::Result, which is an enum that has the variants Ok or Err.
            // The Ok value means all went well, and the Err value means there was some kind of problem.
            // The expect() method is one of the ways you can choose what to do with a Result.
            .expect("Failed to read line");

        // shadowing the previous value of guess with a new one
        // trim() eliminates whitespace and newlines
        // parse() returns a Result type, which is an enum with variants Ok and Err
        let guess: u32 = match guess.trim().parse() {
            // The underscore, _, is a catchall value; in this example, we’re saying we want to match all Err values, no matter what information they have inside them.
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number! \n");
                continue;
            }
        };

        // std::cmp::Ordering is an enum with variants Less, Greater, and Equal
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
