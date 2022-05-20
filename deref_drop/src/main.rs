struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}


fn main() {
    let c  = CustomSmartPointer { data: String::from("my stuff")};
    let d = CustomSmartPointer { data: String::from("other stuff")};

    println!("CustomSmartPointer created.");
    /*      Rust doesn’t let us call drop explicitly because Rust would still automatically call drop on the value at the end of main. 
    This would be a double free error because Rust would be trying to clean up the same value twice.
    */
    //  c.drop();
    drop(c);
    println!("CustomSmartPointer dropped before the end of main")
}
