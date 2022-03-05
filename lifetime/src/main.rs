fn main() {
    // let string1 = String::from("abcd");
    // let string2 = String::from("xyz");

    // let result = longest( string1.as_str(), string2.as_str() );
    // println!("The longest string i {}", result);

    let result;
    let string1 = String::from("abcd");
    { 
        let string2 = String::from("xyz");
        result = longest( string1.as_str(), string2.as_str() );
    }
    println!("The longest string i {}", result);


    #[derive(Debug)] // add this to debug
    struct ImportantExcerpt<'a>{
        part: &'a str,
    }
    

    let novel = String::from("Call me Kata, Some years ago...");
    let first_sentence = novel.split('.').next().expect("Coulnd't find any novel");

    let i;
    {
       i =  ImportantExcerpt{
           part: first_sentence
       };
    }
    
    println!("{:?}", i);

}

// 1. Each parameter that is a reference gets its own lifetime parameter
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output
        // lifetime parameter
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut lef the lifetime of self is 
        // assigned to all output lifetime parameters.

fn first_word<'a>( s: &'a str) -> &'a str{
    let bytes = s.as_bytes();

    for( i , &item ) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// &i32     // a reference
// &'a i32  // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime
fn longest<'a>(x: &'a str, y: &str) -> String {
    let result = String::from("Very long string result");
    result
}


fn borrow_checker () {
    let r ;

    {
        let x = 5;
        let y;
        // r = &x; // cannot assign this scoped variabe to external variable;
        y = &x; // still allowed because y is still at the same scope
    }
    r = 30;
    println!("r: {}", r)
}
