fn main() {
    let s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    // references are immutable by default (you can't change the value of what they refer to)
    // s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!let t = first_word(&s);

    let s1 = String::from("hello world");

    let hello = &s1[0..5];
    let world = &s1[6..11];

    let slice = &s1[0..s1.len()];

    println!("{} {} {}", hello, world, slice);

    // Other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    println!("{} {}", slice[0], slice[1]);
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // b' ' is a byte literal (a single byte)
            return &s[..i]; // return a slice of the string from the start to the index of the space
        }
    }

    // s.len()
    &s[..]
}
