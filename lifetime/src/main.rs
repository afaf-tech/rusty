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


}

// &i32     // a reference
// &'a i32  // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x 
    }else {
        y
    }
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
