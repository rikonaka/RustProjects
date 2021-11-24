/*
fn largest_fun(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}
*/

/*
fn largest_fun<T>(list: &[T]) -> T {
    let mut largest = &list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    *largest
}
*/

struct Point<T> {
    x: T,
    y: T,

}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

struct Point3 {
    x: f32,
    y: f32,
}

impl Point3 {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    /*
    let number_list = vec![1, 2, 3, 4, 5];
    let largest = largest_fun(&number_list);    
    */
    let interger = Point{x: 5, y: 5};
    let float = Point{x: 5.0, y: 6.0};
    let interger_and_float = Point2{x: 1, y: 2.1};

    println!("the result: {}-{}", interger.x, interger.y);
    println!("the result: {}-{}", float.x, float.y);
    println!("the result: {}-{}", interger_and_float.x, interger_and_float.y);
    println!("Point.x = {}", interger.x());

    let dis = Point3{x: 2., y: 3.};
    println!("dis {}", dis.distance_from_origin());
}
