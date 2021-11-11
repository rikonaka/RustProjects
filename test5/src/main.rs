use std::collections::HashMap;

fn main() {
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);

    for vv in v.iter() {
        println!("{}", vv);
    }

    let y: &i32 = &v[2];
    println!("{}", y);
    
    match v.get(2) {
        Some(y) => println!("get from match {}", y),
        None => println!("no value found"),
    }

    enum Spreedsheetcell {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let row = vec![
        Spreedsheetcell::Int(3),
        Spreedsheetcell::Float(10.12),
        Spreedsheetcell::Text(String::from("test")),
    ];

    for i in &row {
        match i {
            Spreedsheetcell::Int(value) => println!("int value {}", value),
            Spreedsheetcell::Float(value) => println!("float value {}", value),
            Spreedsheetcell::Text(value) => println!("text value {}", value),
        }
    }

    let mut s = String::new();
    let data = "some data";
    let mut data_s = data.to_string();
    s.push_str("test");
    data_s.push_str(" 123");

    let s3 = s + " " + &data_s;

    // println!("s value {}", s);
    println!("data {}", data);
    println!("data_s {}", data_s);
    println!("s3 {}", s3);

    let s4 = format!("{}-{}-{}", s3, data, data_s);
    println!("{}", s4);

    let hello = "Здравствуйте";
    for c in hello.chars() {
        println!("char {}", c);
    }

    for b in hello.bytes() {
        println!("byte {}", b);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for s in scores.iter() {
        println!("{}-{}", s.0, s.1);
    }

    /*
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_score = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.iter().zip(initial_score.iter()).collect();
    for s in scores.iter() {
        println!("{}-{}", s.0, s.1);
    }
    let s = scores.get(&String::from("Blue"));
    match s {
        Some(sv) => println!("{}", sv),
        None => println!("None"),
    }
    */

    scores.insert(String::from("White"), 30);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}