use std::env;
use std::process;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let config = lib::Config::new(&args).unwrap_or_else(|err| {
        println!("failed to get arg: {}", err);
        process::exit(1);
    });
    println!("Query {}", config.query);
    println!("In file {}", config.filename);
    // println!("\n");

    if let Err(e) = lib::run(config) {
        println!("application error: {}", e);
        process::exit(1);
    }
}
