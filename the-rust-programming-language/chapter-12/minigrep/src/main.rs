use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Query: {}", query);
    println!("Filename: {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("An error occured while reading the file.");

    println!("\n{}\n", filename);
    println!("\n\n{}", contents);
}