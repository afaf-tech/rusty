fn main() {
// VARIABLE AND MUTABILITY
/*     // let mut x: i32 = 5;
    let x: i32 = 5;
    println!("The value of x is : {}", x);
    // x = 6;
    let x ="six";
    println!("The value of x is : {}", x);

    const SUBSCRIBER_COUNT : u32 = 100_000; */

// DATA TYPES
 // Integers
 let a = 98_222; // Decimal
 let b = 0xff; // Hex
 let c = 0o77; // Octal
 let d = 0b1111_000; // Binary
 let e = b'A'; // Byte (u8 only)

 let f: u8 = 255;
 // unsigned : can only be positive;

 // Floating-point numbers
 let f = 2.0;
 let g: f32 = 3.0;

 // addition
 let sum = 5+ 10;
 //substraction
 let difference = 95.5 - 4.3;
 // multiplication
 let product = 4 * 30;
 // division
 let quotient = 56.7 / 32.2;
 // remainder
 let remainder = 43 % 5;
 
 // Booleans 
 let t = true;
 let f : bool = false;

 // Character 
 let c = 'z';
 let z = 'Z';
 let heart_eyed_cat = 'c'; // dont know how to write it with unicode

 // Compound Types
 let tup = ("Let's get rusty!", 100_000);
 let (channel, sub_count) = tup; // it's like destructuring in Javascript
 let sub_count = tup.1; // access first element of tup;

 let error_codes = [200, 404, 500];
 let not_found = error_codes[1];

 let byte = [0;8];

 // Function
 let sum = my_function(32,32);
 println!("The sum is : {}", sum);

 // Control Flow
 let condition = true;
 let number = if condition {5} else{ 6};

 if number < 0 {
     println!("first condition was true");
 }else if number < 22 {
     println!("second condition was true");
 } else { 
     println!("condition was false");
 }

 let mut counter = 0;
 let result = loop {
     counter += 1;
     println!("again"); // will not stop until we call break
     if counter == 10 {
         break counter; // break as well as returning the counter;
     }
 };

 println!("The result is : {}", result);

 let mut number = 3;
 while number != 0{
     println!("{}!", number);

     number -=1;
 }

 let a = [1,2,3,4,5,6];

 for element  in a.iter(){
     println!("the value is : {}", element)
 }
 // loop number from  1 to 3
 for number in 1..4{
     println!("the value is : {}", number)
 }

 // Line comment
 /* 
    Block comment
 */

}

fn my_function(x : i32, y: i32) -> i32 {
    println!("Another function.");
    println!("The value of x is : {}", x);
    println!("The value of y is : {}", y);

    // returning
    x + y

}
