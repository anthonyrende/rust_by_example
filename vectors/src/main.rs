fn main() {
    // Rust can infer the type because of the initial i32 values
    let v1 = vec![1, 2, 3];

    let mut v: Vec<i32> = Vec::new();

    v.push(4);
    v.push(5);
    v.push(6);

    println!("Vec: {:?}", v);

    // There are two ways to reference a value stored in a vector: via indexing or using the get method.
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    // when we use the get method with the index passed as an argument, we get an Option<&T>
    match third {
        Some(third) => println!("The third element is {}!", third),
        None => (),
    }

    let first = &v[0];

    // v.push(6); <-- wont complie because you can’t have mutable and immutable references in the same scope

    // println!("The first element is: {first}");
}
