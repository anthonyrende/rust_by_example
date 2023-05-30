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
}
