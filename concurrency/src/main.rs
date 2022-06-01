use std::thread;
use std::time::Duration;

fn main() {
    // let handle = thread::spawn(move || {
    //     for i in 0..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }


    // handle.join().unwrap();



    let v = vec![1, 2, 3];

    let handle = thread::spawn( move || { // force clojure to take ownership of the value
        println!("Here's a vector: {:?}", v);
    });

    //Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates
    handle.join().unwrap(); 
    

}