struct User {
    username  : String,
    email : String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32,i32);
struct Point(i32, i32, i32);

#[derive(Debug)] // add this to debug
struct Rectangle {
    width:u32,
    height: u32,
}

// for method
impl Rectangle {
    fn area(&self) ->u32{
        self.width * self.height
    }

    fn can_hold(&self, other : &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}


// behaves like static method
impl Rectangle {
    fn square(size: u32) -> Rectangle{
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let mut user1 = User{
        email : String::from("afaf@gmail.com"),
        username : String::from("afaf"),
        active : true,
        sign_in_count: 1,
    };

    let name = user1.username;
    println!("{}", name);
    user1.username = String::from("wallstreet");
    
    let user2 = build_user(String::from("key@gmail.com"), String::from("kyle23"));
    println!("{}", user2.email);
    
    let user3 = User {
        email : String::from("user3@gmail.com"), 
        username : String::from("user3"), 
        ..user2 // like spread operator in javascript
    };

    // calculate area 
    let width1 = 30;
    let height1= 50;
    let rect: Rectangle= Rectangle {
        width: width1,
        height: height1,
    };
    let rect1: Rectangle= Rectangle {
        width: 20,
        height: 10,
    };
    let rect2: Rectangle= Rectangle {
        width: 50,
        height: 400,
    };
    let rect3: Rectangle= Rectangle::square(32); // ilke static method

    println!("rect can hold rect1 {}", rect.can_hold(&rect1));
    println!("rect can hold rect2 {}", rect.can_hold(&rect2));

    println!("rect {:?}", rect);
    println!("The area of the rectangle is {} square pixels. with function external", area(&rect));
    println!("The area of the rectangle is {} square pixels. with method", rect.area());
}

fn build_user(email: String, username: String)-> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


fn area_old(dimensions: (u32, u32))-> u32 {
    dimensions.0 * dimensions.1
}

fn area(rectangle: &Rectangle)-> u32 {
    rectangle.width * rectangle.height
}