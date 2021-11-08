#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width >= other.width && self.height >= other.height;
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };

    println!("{}", rect1.area());

    let rect2 = Rectangle {
        width: 100,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("can hold: {}", rect1.can_hold(&rect2));
    println!("can hold: {}", rect1.can_hold(&rect3));
}
