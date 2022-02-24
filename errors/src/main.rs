use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;
use std::net::IpAddr;
fn main() {
    a();
    read_username_from_file();

    let home: IpAddr = "127.0.0.1".parse().unwrap();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hellothere.txt")?.read_to_string(&mut s);
    println!("{}", s);
    Ok(s)
}

fn a(){
    let f =  File::open("hello.txt");
    // manual checking
    let f = match f{
        Ok(file) => file,
        // Err(errror)=> panic!("Problem opening file: {:?}", errror)
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("Problem creating file: {:?}", err)
            },
            other_error => panic!("problem opening file: {:?}", other_error)
        }
    };

    // aumatic cheking 
    // let f =  File::open("hellothere.txt").expect("Failed to open hellothere.txt");



    b();
}

fn b(){
    c(22)
}

fn c(num: i32){
    if num == 2 {
        panic!("Don't pass in 11!")
    }
}
