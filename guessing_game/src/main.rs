use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    
    loop {
        let secret_number = rand::thread_rng().gen_range(1, 101);
    
        println!("The secret_number is : {}", secret_number);
        println!("Please input your guess.!");

        let mut guessed_number: String = String::new();

        io::stdin()
        .read_line(&mut guessed_number)
            .expect("Failed to read_line");
        
        let guessed_number: u32= match guessed_number.trim().parse(){
            Ok( num) => num,
            Err(_) => continue, // ignore when you input invalid number;
        };
            
        println!("You guessed: {}", guessed_number);
        
        match guessed_number.cmp(&secret_number){
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}","Too Big".red()),
            Ordering::Equal => {
                println!("{}", "You Win".green());
                break;
            },
        }
    }
}
