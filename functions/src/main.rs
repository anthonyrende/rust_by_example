fn main() {
    println!("Hello, world!");
    another_function(5, 'h');

    // Statements vs Expressions
    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resulting value.

    // let x = (let y = 6); // error: expected expression, found statement (`let`)

    let x = 5;
    let y = {
        //  Expressions do not include ending semicolons.
        // If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value
        let x = 3;
        x + 1 // no semicolon
    };
    println!("The value of y is: {}", y);

    // Functions with return values
    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // x + 1; // error: mismatched types: expected `i32`, found `()`
    x + 1
}

fn another_function(num: i32, unit_label: char) {
    println!("The measuerment is: {num}{unit_label}");
}
