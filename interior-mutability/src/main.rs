/* Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; 
normally, this action is disallowed by the borrowing rules. */

/* Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:

- Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
- Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; 
    RefCell<T> allows immutable or mutable borrows checked at runtime.
- Because RefCell<T> allows mutable borrows checked at runtime, 
    you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable. */


    // Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>

#[derive(Debug)]
enum List         {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    //We wrap the list a in an Rc<T> so when we create lists b and c, they can both refer to a
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(5)), Rc::clone(&a));


    // After we’ve created the lists in a, b, and c, we add 10 to the value in value. We do this by calling borrow_mut on value,
    // which uses the automatic dereferencing feature
    *value.borrow_mut() += 10; // https://doc.rust-lang.org/book/ch05-03-method-syntax.html#wheres-the---operator

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}