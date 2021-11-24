use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick here.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick here.\nDuct type.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick here.\Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_iinsensitive(query, contents));
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // return vec![];
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    return result;
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arg");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        return Ok(Config { query, filename });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("poem contents:\n{}", contents);
    for line in search(&config.query, &contents) {
        println!("line: {}", line);
    }
    return Ok(());
}
