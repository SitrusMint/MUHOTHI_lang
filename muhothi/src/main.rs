mod bf;
use bf::{execute_bf};

use std::env;
use std::fs;

fn main() { 
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("incorrect argument")
    }

    let content = fs::read_to_string(&args[1]).unwrap();

    if let Err(e) = execute_bf(&content) {
        println!("{}", e);
        std::process::exit(1);
    }
}