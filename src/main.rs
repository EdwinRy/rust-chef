mod chef;
mod vm;

use std::env;
use chef::ChefProgram;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = args.first().expect("Expected filename");

    let program = ChefProgram::new(file);
    println!("Hello, world!");
}
