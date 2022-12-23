fn greet(name: String) {
    println!("Hello, {}", name)
}

fn main() {
    let name = String::from("Joe");
    greet(name);
    // Can we call greet again?
    // If you try to call greet again, throws "value borrowed here after move"
    println!("Let's try greeting {} again!", name)
    //name has not been move and is owned by greet()
    //To work around this issue, we need to clone the name variable
}
