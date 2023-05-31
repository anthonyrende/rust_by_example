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
}
