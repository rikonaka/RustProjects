use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("1.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => return Ok(s),
        Err(e) => return Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("1.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    return Ok(s);
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("wrong input {}", value);
        }

        Guess {
            value,
        }
    }

    pub fn value(&self) -> i32 {
        return self.value;
    }
}

fn main() {
    /*
    let f = File::open("1.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("1.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("error {:?}", error),
            },
            _ => panic!("other error"),
        },
    };
    */
    // let f = File::open("1.txt").unwrap();
    // let f = File::open("1.txt").expect("can not open");
    match read_username_from_file() {
        Ok(s) => println!("{}", s),
        Err(e) => panic!("{:?}", e),
    }

    match read_username_from_file_2() {
        Ok(s) => println!("{}", s),
        Err(e) => panic!("{:?}", e),
    }
}
