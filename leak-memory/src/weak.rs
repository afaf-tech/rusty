// The Rc<T> type uses weak_count to keep track of how many Weak<T> references exist, similar to strong_count. 
//The difference is the weak_count doesn’t need to be 0 for the Rc<T> instance to be cleaned up.

use std::cell::RefCell;
use std::rc::{Rc,Weak};

/* , a parent node should own its children: if a parent node is dropped,
 its child nodes should be dropped as well.
  However, a child should not own its parent: if we drop a child node,
   the parent should still exist. 
This is a case for weak references! */
#[derive(Debug)]
struct Node  {
    value : i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn weak_main() {
    println!("=== weak section ===");
    let leaf = Rc::new(Node{
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "[global] leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),    
    );

    {

        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)])
        });
    
        //  calling Rc::downgrade increases the weak_count by 1
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);


        println!(
            "[scope] branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        //we’ll see it will have a strong count of 2, 
        //because branch now has a clone of the Rc<Node> of leaf stored in branch.children
        println!(
            "[scope] leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    

    println!("[global] leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "[global] leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),    
    );
}

