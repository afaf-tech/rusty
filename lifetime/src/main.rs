fn main() {

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
