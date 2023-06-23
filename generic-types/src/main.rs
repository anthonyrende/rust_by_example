/*
largest is generic over some type T. 

This function has one parameter named list, which is a slice of values of type T. 
The largest function will return a reference to a value of the same type T.
 */
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
//  Point<T, U> generic over two types so that x and y can be values of different types
struct Point<T, U> {
    x: T,
    y: U,
}

// We can define enums to hold generic data types in their variants
enum Option<T> {
    Some(T),
    None,
}
// When you recognize situations in your code with multiple struct or enum definitions that differ only in the types of the values they hold, you can avoid duplication by using generic types instead.
enum Result<T,E> {
    Ok(T),
    Err(E),
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10};
    let float = Point { x: 1.0, y: 3.0}; 

    println!("{:?} {:?} ", integer, float);
}
