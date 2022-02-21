mod front_of_house; // will get content from another file with the same as this

fn serve_order(){}

mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::serve_order();
    }

    fn cook_order(){}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str)-> Breakfast {
            Breakfast { 
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    //  enum
    pub enum Appetizer {
        Soup,
        Salad,
    }
}


pub fn eat_at_restaurant(){
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");


    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


// like import so that we don't have to specify the path manually
use self::front_of_house::hosting;
use rand::{Rng, CryptoRng, ErrorKind::Transient};
// use std::io::{self, Write};
use std::io::*;

pub fn eat_at_restaurant2(){
    let secret_number = rand::thread_rng().gen_range(1, 100);

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();


}


