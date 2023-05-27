// THe rules of references:
// 1. At any given time, you can have either one mutable reference or any number of immutable references.
// 2. References must always be valid.

fn main() {
    let s1 = String::from("hello");

    // The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it
    // because it does not own it, **the value it points to will NOT be dropped** when the reference goes out of scope
    let len = calculate_length(&s1); // &s1 is a reference to s1

    println!("The length of '{}' is {}.", s1, len);

    let s = String::from("hello");

    change(&s); // &s is a reference to s

    let mut s2 = String::from("hello");

    change_mut(&mut s2); // &mut s2 is a mutable reference to s2

    let r1 = &mut s2;
    let r2 = &mut s2; // <-- this will not compile : you cannot have a reference to a mutable variable (it is already borrowed)

    // println!("{}, {}", r1, r2); <-- Rust prevents this problem by refusing to compile code with data races! ( 2 or more pointers access the same data at the same time )

    let mut s3 = String::from("hello");

    let r1 = &s3; // no problem
    let r2 = &s3; // no problem
    let r3 = &mut s3; // BIG PROBLEM
                      // println!("{} and {} and {}", r1, r2, r3); <-- this will not compile : you cannot have a mutable reference while you have an immutable one
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String (it does not take ownership)
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &String) {
    // some_string.push_str(", world"); <-- this will not compile : you cannot modify something that is borrowed
    println!("{}", some_string);
}

fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
    println!("{}", some_string);
}
