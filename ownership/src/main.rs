fn main() {
    // ---------------- OWNERSHIP RULES ----------------
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 2. When the owner goes out of scope, the value will be dropped.

    { // s is not valid here, it's not yet declared
        // automatically add s below on the heap memory by rust
        let s = String::from("Hello"); // s is valid from this point forward.
        // do stuff with s
    } // this scope is now over, and s is nolonger valid

    let x = 5;
    let y = x; // Copy, exceptional (rust do copying instead of moving for simple data type like int and char)
    println!("{}", x);

    let s1 = String::from("Hello");
    let s2 = s1.clone(); // Move (not shallow copy)
    
    // s1 will not be shown due to having been deleted.
    // use s1.clone() to prevent error by cloning it.
    println!("{}, world!", s1);  
   
    let c1 = '1';
    let c2 = c1;
    println!("{}, world!", c1);

    // ownership through Function
    let s = String::from("Hello");
    takes_ownership(s);
    // println!("{}, world!", s); // not allowed

    let x = 32;
    makes_copy(x);
    println!("{}, world!", x);

    let string1 = String::from("Nano");
    let len = calculate_length(&string1); // passing by reference
    println!(" string {}, len, {}!", string1, len);

    let mut string_mut = String::from("Fikri");
    change(&mut string_mut);

    let s1BorrowMut = &mut string_mut;
    let s2BorrowMut = &mut string_mut;

    println!("{}", string_mut);

    let r3 = &mut string_mut;
    println!("{}, world!", r3);


    // Dangeling References
    // The Rule of References
    // 1. At any given time, you can have either one mutable reference
     // or any number of immutable references.

    // 2. References must always be valid memory. 



    // Slice type
    let mut s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    let slice_from_entire = &s[..];
    let word = first_word(&slice_from_entire);
    
    println!("{}", word);
    println!("{}", slice_from_entire);
    s.clear();

}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

fn calculate_length( s : &String) -> usize { // passed by reference
    let length = s.len();
    length
}

fn change (some_string: &mut String){
    some_string.push_str(", Handsome");
}

/* fn dangle()-> &String{ // cannot do this because after function done. 
    //all the variable inside this in memory on the heap will be deleted
    let s = String::from("Hello");

    &s
} */


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item ) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}
