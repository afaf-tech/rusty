use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect();

    let query = &args[1]; // the first[0] will be the binary path
    let filename = &args[2]; // 
    // println!("{:?}", args);

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
            .expect("Failed to read"); // error message
    
    println!("With text:\n{}", contents);
}
