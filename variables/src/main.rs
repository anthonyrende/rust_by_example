fn main() {
    let x = 5;

    // shadowing the previous value of x
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
        // inner scope returns and the value of x is 12
    }
    // outer scope returns and the value of x is 6
    println!("The value of x is: {}", x);
}
