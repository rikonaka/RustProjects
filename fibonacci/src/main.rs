use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("input not correct");
        process::exit(1);
    }

    let fib_len: usize = args[1].trim().parse().expect("wrong input!");

    let mut fib_vec: Vec<f64> = Vec::new();
    fib_vec.push(0.);
    fib_vec.push(1.);

    for i in 2..fib_len {
        fib_vec.push(fib_vec[i-1] + fib_vec[i-2]);
    }

    for (i, &fib) in fib_vec.iter().enumerate() {
        println!("value-{}: {}", i, fib);
    }

}
