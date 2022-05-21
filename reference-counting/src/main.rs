// bad case
// enum List {
//     Cons(i32, Box<List>),
//     Nil
// }
enum List {
    Cons(i32, Rc<List>),
    Nil
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    
    // ERROR
    // let b = Cons(3, Box::new(a)); // value moved here
    // let c = Cons(6, Box::new(a)); // value used here after move
    
    // Solution 
    // Referece Counting Rc<T>
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    
}
