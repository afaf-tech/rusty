use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();

    let query = &args[1]; // the first[0] will be the binary path
    let filename = &args[2]; // 
    // println!("{:?}", args);

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
