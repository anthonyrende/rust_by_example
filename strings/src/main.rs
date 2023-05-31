fn main() {
    /* The String type, which is provided by Rust’s standard library rather than coded into the core language,
    is a growable, mutable, owned, UTF-8 encoded string type.

    Rust has only one string type in the core language, which is the string slice str (&str)
    */

    // Created a new empty String
    let mut s = String::new();

    let data = "initial contents";

    let d = data.to_string();

    // the method also works on a literal directly:
    let d = "initial contents".to_string();

    // We can also use the function String::from to create a String from a string literal

    let s = String::from("initial contents");

    // Updating a String

    // Appending to a String with push_str and push
    let mut s = String::from("hello");
    s.push_str(" world!");

    println!("{}", s);

    // The push method takes a single character as a parameter and adds it to the String. Listing 8-17 adds the letter “l” to a String using the push method.
    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    /*
    The reason s1 is no longer valid after the addition, and the reason we used a reference to s2,
        has to do with the signature of the method that’s called when we use the + operator.
    The + operator uses the add method, whose signature looks something like this:
        fn add(self, s: &str) -> String {
     */
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("s3: {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
}
