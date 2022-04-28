// Imports
use std::env;

// Start of Main
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
