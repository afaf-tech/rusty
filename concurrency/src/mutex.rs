// Mutual Exclusion

//Mutexes have a reputation for being difficult to use because you have to remember two rules:

use std::rc::Rc;
// You must attempt to acquire the lock before using the data.
// When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.
use std::sync::Mutex;

pub fn mutex_single_threaded(){

    println!("mutex single threaded");
    let m = Mutex::new(5);
    
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

}

use std::thread;
use std::sync::Arc;  // Atomic Reference Counting : atomics work like primitive types but are safe to share across threads.


pub fn mutex_multiple_threads() {
    println!("mutex multi threaded");

    // let counter = Mutex::new(0);
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}