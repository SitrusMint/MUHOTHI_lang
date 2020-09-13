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
