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