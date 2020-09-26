mod lib;

use lib::{execute_bf};
use std::env;
use std::fs;

fn main() { 
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("incorrect argument")
    }

    let content = fs::read_to_string(&args[1]).unwrap();

    match execute_bf(&content) {
        Ok(_) => {},
        Err(e) => println!("{}", e),
    }
}