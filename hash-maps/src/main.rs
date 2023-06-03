use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // Like vectors, all of the keys must have the same type as each other (and all of the values must have the same type)
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    println!("Scores: {:?}", scores);

    let team_name = String::from("Blue");
    // The get method returns an Option<&V>
    //  copied() returns an Option<i32> (rather than an Option<&i32>)
    //  unwrap_or to set score to zero if scores doesn't have an entry for the key.
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Access score value: {:?}", score);

    for (key, value) in &scores {
        println!("Key {key}: Value: {value}");
    }

    // Updating a Hash Map
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("map: {:?}", map);

    // Adding a Key and Value Only If a Key Isn’t Present
    scores.entry(String::from("Yellow")).or_insert(50);

    println!("Scores: {:?}", scores);

    // Updating a Value Based on the Old Value
    let text = "Hello world wonderful world";

    let mut new_map = HashMap::new();

    for word in text.split_whitespace() {
        // The or_insert method returns a mutable reference (&mut V)
        // we store that mutable reference in the count variable
        let count = new_map.entry(word).or_insert(0);
        // in order to assign to that value, we must first dereference count
        *count += 1;
    } 
    println!("new map: {:?}", new_map);
}
