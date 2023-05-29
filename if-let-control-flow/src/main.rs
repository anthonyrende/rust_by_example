fn main() {
    /*
    let config_max = Some(3u8);
     match config_max {
         // If Some value,  bind the value to the variable max
         Some(max) => println!("The maximum is configured to be {}", max),
         _ => (),
     }

     Instead, we could write this in a shorter way using if let */

    // let config_max = Some(3u8);
    let config_max = Some("a");
    if let Some(max) = config_max {
        println!("The max value is {}", max)
    } else {
        println!("No value")
    }
    /*
    You lose the exhaustive checking that match enforces, but less boilerplate - depends on use case
    In other words, if let is syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
    */
}
