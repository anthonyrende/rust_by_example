use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", error);
            }
        }
    };

    // Shortcuts for Panic on Error: unwrap and expect

    // If the Result value is the Ok variant, unwrap will return the value inside the Ok. 
    // If the Result is the Err variant, unwrap will call the panic! macro 
    let greeting_file = File::open("hello.txt").unwrap();

    // Similarly, the expect method lets us also choose the panic! error message. 
    // Using expect instead of unwrap and providing good error messages can convey your intent
    //  and make tracking down the source of a panic easier
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

}

// Propagating Errors

// the function is returning a value of the type Result<T, E> - the generic parameter T is concrete type String, 
// and the generic type E has been filled in with the concrete type io::Error.
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    // we handle the Result value from "user_name_file_result" with a match
    let mut username_file = match username_file_result {
        // If Ok, the file handle in the pattern variable {file} becomes the value in the mutable variable {username_file}
        Ok(file) => file,
        //  In the Err case, instead of calling panic!, we use the return keyword to return early out of the function entirely,
        // and pass the error value from File::open
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    // read the contents of the file into username
    match username_file.read_to_string(&mut username) {
        // The read_to_string method also returns a Result because it might fail
        Ok(_) => Ok(username),
        // we don’t need to explicitly say return, because this is the last expression in the function.
        Err(e) => Err(e),
    }

    /*
    The code that calls this code will then handle getting either an Ok value that contains a username or an Err value that contains an io::Error. 
    It’s up to the calling code to decide what to do with those values. If the calling code gets an Err value, it could call panic! and crash the program,
    use a default username, or look up the username from somewhere other than a file, for example
     */
}



// A Shortcut for Propagating Errors: the ? Operator

// The ? placed after a Result value is defined to work in almost the same way as the match
// If the value of the Result is an Ok, the value inside the Ok will get returned
// If the value is an Err, the Err will be returned from the whole function,
//  as if we had used the return keyword so the error value gets propagated to the calling code.
fn _read_username_from_file() -> Result<String, io::Error> {
    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

// The ? operator eliminates a lot of boilerplate and makes this function’s implementation simpler.
    let mut username = String::new();

    //  we’re only allowed to use the ? operator in a function that returns Result, Option, or another type that implements FromResidual.
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}