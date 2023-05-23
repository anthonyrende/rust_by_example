fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    counter();
    while_loop();
    for_in();
}

fn for_in() {
    let list = [1, 2, 3, 4, 5];

    for element in list {
        println!("the value in the list is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn while_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut number = 3;
    let mut index = 0;
    while index < 5 {
        println!("{}!", a[index]);

        index += 1;
    }

    println!("LIFTOFF!!!");
}

fn counter() {
    let mut counter: i32 = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    counter;

    println!("\nThe result is {}, counter = {counter}", result);
}
