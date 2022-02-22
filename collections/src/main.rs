fn main() {
    let a = [1,2,3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let mut v2 = vec![1,2,3, 4,5,6];
    let third = &v2[2];

    // v2.push(13); not allowed
    println!("The third element is:{}", third);

    match v.get(20){
        Some(third) => println!("The third element is:{}", third),
        None => println!("There is no third element")
    }

    for i in &mut v2 {
        *i += 50;
        println!("{}", i);
    }

    // each value has been added with 50;
    for i in & v2 {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not an integer")
    }


    // Strings are stored as a colection of UTF-8 encoded bytes
    let  s1 = String::new();
    let s2 = String::from("red");
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    println!("{}", s); // foobar!
    let s3 = s1 + &s2;
    // println!("{}", s1); // cannot borrow value after move
    println!("{}", s2); // can borrow value because used by ref

    // concat with format macro
    let s4 = format!("{}{}", s3, s2);


    // HASHMAP
    use std::collections::HashMap;

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    // println!("{}", blue); // value of blue has been borrowed by scores when inserting 

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key , value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("red"), 21);
    scores.insert(String::from("red"), 21);

    scores.entry(String::from("green")).or_insert(32);
    scores.entry(String::from("green")).or_insert(33);


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // will add "word" as a key and to map or increment its value
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}   
