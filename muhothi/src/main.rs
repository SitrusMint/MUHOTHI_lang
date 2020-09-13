<<<<<<< HEAD
use lib::{execute_bf};
use std::env;
use std::fs;
use muhothi::bf::execute_bf;

fn main() -> Result<(), &str> {
    let args: Vec<String> = env::args().collect();
    let file_name = args[1];
    let content = fs::read_to_string(file_name).expect("file not found");
    execute_bf(&content)
}
=======
mod lib;

use lib::{execute_bf};
use std::env;
use std::fs;

fn main() -> Result<(), ()>{
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(&args[1]).expect("file couldn't be open");

    if let Ok(()) = execute_bf(&content) {
        Ok(())
    } else {
        Err(())
    }
}
>>>>>>> 0291b35dfd35aa8fb21fb0793bd59a66195d470a
