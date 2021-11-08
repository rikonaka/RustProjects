use std::option::Option;

enum IpAddr {
    V4(String),
    V6(String),
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => {
            println!("here");
            return Some(i+1);
        },
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5);
    let some_number_1 = plus_one(some_number);
}
