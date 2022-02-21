#[derive(Debug)] // add this to debug
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move{ x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message{
    fn some_function (){
        println!("let's get fikri")
    }
}
struct IpAddr {
    kind: IpAddrKind,
    address : String,
}


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddrKind::V4(127, 0,0,1);
    println!("{:?}", localhost);

    let x :i8 = 5;
    let y : Option<i8> = None;

    let sum = x + y.unwrap_or(0);
    println!("{}", sum);

    // usage
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // additional information
    let some_value = Some(3);
    // if use match we have to provide handling for None result. TIRING wkwk
    match some_value{
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_value{
        println!("three");
    }
}


fn plus_one(x : Option<i32>) -> Option<i32>{
    match x {
        Some(i) => Some(i+1),
        _=> None
    }
}
