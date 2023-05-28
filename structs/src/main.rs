struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32); // tuple struct
struct Point(i32, i32, i32); // tuple struct

// Unit struct are useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself
struct AlwaysEqual; // unit struct

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someEmail@test.com"),
        sign_in_count: 1,
    };

    //  If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field
    user1.email = String::from("anotherEmail@test.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("anotherExample@test.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Using struct update syntax, we can achieve the same effect with less code,
    let user2 = User {
        email: String::from("another@examples.com"),
        // remaining fields should get their values from the corresponding fields in user1
        ..user1
    };

    println!("user1: {}", user2.email); // <-- won't compile because user1 was moved to user2 (user1 is no longer valid)

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    //  If the parameter name and the struct field name are the same, we only need to write the parameter name (field init shorthand syntax)
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
