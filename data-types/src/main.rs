use std::io;

fn main() {
    /* Scalar Types
    // A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

     Integers

     Length	Signed	Unsigned
     8-bit	i8	u8
     16-bit	i16	u16
     32-bit	i32	u32
     64-bit	i64	u64
     128-bit	i128	u128

    // Signed vs. Unsigned = whether it can be negative or positive

     arch	isize	usize
     64-bit	x86_64	x86_64
     32-bit	i686	i686

     The isize and usize types depend on the kind of computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

     */

    /*
    Compound Types
    (Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.)

    Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Destructuring
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // Accessing tuple elements directly by using a period (.) followed by the index of the value we want to access.
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    /*
    Arrays
    Unlike a tuple, every element of an array must have the same type.
    Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples.

    Arrays are useful when you want your data allocated on the stack rather than the heap.
    */
    let a = [1, 2, 3, 4, 5];
    // If you try to access an element of an array using an index value that is greater than or equal to the array length, Rust will prevent your program from compiling.
    // let index = 10;

    let index = 4;
    let element = a[index];
    println!("The value of element is: {}", element);

    let a = [1, 2, 3, 4, 5];

    println!("\n Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
