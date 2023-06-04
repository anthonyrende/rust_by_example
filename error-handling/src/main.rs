use std::fs::File;
use std::io::ErrorKind;

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
