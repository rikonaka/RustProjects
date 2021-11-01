use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("not enough args!");
        process::exit(1);
    }

    let mode = &args[1];
    let value = &args[2];

    // println!("mode is {}", mode);
    // println!("value is {}", value);

    let mode: u32 = mode.trim().parse().expect("failed to get mode");
    let value: f32 = value.trim().parse().expect("failed to get value");

    if mode == 0 {
        // 到摄氏温度
        let result_value = (5.0 * (value - 32.0)) / 9.0;
        println!("result is {}", result_value);
    } else if mode == 1 {
        // 到华氏温度
        let result_value = (9.0 * value) / 5.0 + 32.0;
        println!("result is {}", result_value);
    } else {
        println!("wrong input mode!");
        process::exit(1);
    }
}
