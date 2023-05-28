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

// #[derive(Debug)] // This annotation enables us to print the struct using {:?} or {:#?} (pretty-print)
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale), // dbg!() macro prints the value of the expression using std::fmt::Debug trait (https://doc.rust-lang.org/std/fmt/trait.Debug.html
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );

//     println!("rect1 {:#?}", rect1);
// }
// //  we want to borrow the struct rather than take ownership of it. This way, main retains its ownership and can continue using rect1, which is the reason we use the & in the function signature and where we call the function.
// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }

//  Method Syntax //

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // we don’t want to take ownership, and we just want to read the data in the struct, not write to it.
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // Getter methods
    // Getters are useful because you can make the field private but the method public, and thus enable read-only access to that field as part of the type’s public API
    fn width(&self) -> u32 {
        self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        // self.width > other.width && self.height > other.height
        self.width > other.width && self.height > other.height
    }

    // Associated Functions
    // Another useful feature of impl blocks is that we’re allowed to define functions within impl blocks that don’t take self as a parameter. These are called associated functions because they’re associated with the struct. They’re still functions, not methods, because they don’t have an instance of the struct to work with. You’ve already used the String::from associated function.
    // Associated functions are often used for constructors that will return a new instance of the struct.
    fn square(size: u32) -> Self {
        // The Self keywords in the return type and in the body of the function are aliases for the type that appears after the impl keyword, which in this case is Rectangle.
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    // &rect2 immutable borrow to rect2 (we can't modify rect2)
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("The width of the rectangle is {} pixels.", rect1.width());

    let square = Rectangle::square(3);
    println!("square {:#?}", square);
}

/*
Structs let you create custom types that are meaningful for your domain.
By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear.
In impl blocks, you can define functions that are associated with your type,
and methods are a kind of associated function that let you specify the behavior that instances of your structs have.
 */
