fn main() {
    let mut s = String::from("hello");
    let len = calculate_length(&s);
    println!("len: {}", len);

    change(&mut s);
    println!("change result: {}", s);

    let search_s = String::from("hello world");
    let search_result_s = first_word(&search_s[..]);
    println!("search result: {}", search_result_s);

    let search_ss = "hello world";
    let search_result_ss = first_word(search_ss);
    println!("search result: {}", search_result_ss);

    let mut user = User {
        username: String::from("xx"),
        email: String::from("xx@xx.com"),
        active: true,
        sign_in_count: 1,
    };

    user.email = String::from("yy@yy.com");

    let user2 = User {
        username: String::from("zz"),
        email: String::from("zz@zz.com"),
        ..user
    };

    println!("username: {}", user.username);
    println!("email: {}", user.email);
    println!("sign_in_count: {}", user2.sign_in_count);
    println!("active: {}", user2.active);

    let black = Color(0, 0, 0);
    let origin = Point(0, 100, 0);

    println!("black: {}", black.0);
    println!("origin: {}", origin.1);
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn change(s: &mut String) {
    s.push_str(" 123");
}