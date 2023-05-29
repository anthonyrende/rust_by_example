#[derive(Debug)]
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn main() {
    let new_coin = Coin::Penny;

    value_in_cents(new_coin);
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Plus one returns {:?}", six);

    let dice_roll = 9;
    match dice_roll {
        3 => do_something(),
        7 => do_something_else(),
        // Matches must be exhaustive so this underscore is a catch-all for every other roll
        // _ => reroll(),
        _ => (),
    }
}

fn do_something() {}
fn do_something_else() {}
fn reroll() {}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky penny!");
            1
        }
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
