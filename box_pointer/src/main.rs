// Box<T> for allocating values on the heap
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>), // if we are not using Box<> it couldn't compile rust doesn't know how much memory to store
    Nil,
}

use List::{Cons, Nil};

// defining my own smart pointer
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// enabling my own smart pointer to support dereferencing (*y)
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    //cons list
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
    

    let x = 5;
    // set y to be an instance of a box pointing to a copied value of 
    // x rather than a reference pointing to the value of x
    // let y = &x;
    let y = MyBox::new(x); 

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // When we entered *y in Listing 15-9, behind the scenes Rust actually ran this code:
    // *(y.deref())

}
