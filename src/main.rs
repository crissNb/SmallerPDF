use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 0 {
        println!("Provide a file path!");
        exit(0);
    }

    // Get file path from the first argument
    let file_path = &args[1];
    println!("Hello, world! {}", file_path);
}
