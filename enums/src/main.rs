// IpAddrKind is a custom data type (enum)
#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),

// We’re also able to define methods on enum
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
// the Option type encodes the very common scenario in which a value could be something or it could be nothing.
enum Option<T> {
    // None, in some sense, means the same thing as null: we don’t have a valid value
    None,
    Some(T),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // let new_route = route(IpAddrKind::V6);
    // println!("route: {:?}", new_route);

    // The name of each enum variant that we define also becomes a function that constructs an instance of the enum
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    println!("{:?}", home)

    // type = Option<i32>
    let some_number = Some(5);
    // type = Option<char>
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}

fn route(ip_kind: IpAddrKind) -> IpAddrKind {
    ip_kind
}
