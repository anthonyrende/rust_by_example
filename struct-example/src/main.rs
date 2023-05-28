// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

/*  The code above is not very expressive. Let's use tuples instead: */

// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

/* this version is less clear: tuples don’t name their elements, so we have to index into the parts of the tuple, making our calculation less obvious. */

// Let's use structs instead:

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // dbg!() macro prints the value of the expression using std::fmt::Debug trait (https://doc.rust-lang.org/std/fmt/trait.Debug.html
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 {:#?}", rect1);
}
//  we want to borrow the struct rather than take ownership of it. This way, main retains its ownership and can continue using rect1, which is the reason we use the & in the function signature and where we call the function.
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
