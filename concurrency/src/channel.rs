use std::sync::mpsc;
use std::thread;

pub fn main_channel(){
    // mpsc was an acronym for multiple producer, single consumer
    let (tx, rx) = mpsc::channel();
    // tx : transmitter channel
    // rx : receiver channel

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // DON'T do this
        // println!("val is {}", val); //Here, we try to print val after we’ve sent it down the channel via tx.send
    });

    let received = rx.recv().unwrap();
    println!("Received : {}", received);
}

use std::time::Duration;

pub fn main_channel_multiple_values() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    /* we’re not calling the recv function explicitly anymore: instead, we’re treating rx as an iterator. 
        For each value received, we’re printing it.
        When the channel is closed, iteration will end. */
    for received in rx {
        println!("Got: {}", received);
    }
}