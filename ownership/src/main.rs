// Memory and Allocation
// With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap,
// unknown at compile time, to hold the contents.
//      This means:

//      The memory must be requested from the memory allocator at runtime.
//      We need a way of returning this memory to the allocator when we’re done with our String.
fn main() {
    {
        let s = "hello"; // s is valid from this point forward
                         // do stuff with s
    } // this scope is now over, and s is no longer valid
      // println!("{}", s); <-- this will not compile : the memory is automatically returned once the variable that owns it goes out of scope

    let mut s = String::from("hello"); // s is valid from this point forward

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // this will print `hello, world!`

    let s1 = String::from("hello");
    // let s2 = s1; <-- this will not compile : s1 is no longer valid because the memory was moved to s2
    // println!("{}, world!", s1); <-- this will not compile : s1 is no longer valid because the memory was moved to s2

    let s2 = s1.clone(); // deep copy (expensive because it copies the heap data)
    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y); // <-- works because integers are stored on the stack and not the heap! (fixed size)

    // Ownership and Functions

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    // println!("{}", s); <-- this will not compile : s is no longer valid because the memory was moved to s2

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward
    println!("{}", x); // <-- works because integers are stored on the stack and not the heap! (fixed size)

    let s1 = String::from("hello"); // s1 comes into scope

    let (s2, len) = calculate_length(s1); // s1 is moved into calculate_length and so is no longer valid here

    println!("The length of '{}' is {}.", s2, len); // s2 is valid here because it was returned by calculate_length which did not take ownership of s1
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
