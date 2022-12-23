fn main() {
    //Shows how to initialize and assign string valuable value
    let name = String::from("Joe");
    println!("Hello, {}!", name);
    //Notice this variable is immutable, any attempt to change value will result in error.
    //name.push_str(" Karasha");
    let mut mutable_name = String::from("Joe");
    mutable_name.push_str(" Karasha!");
    println!("Hello, {}!", mutable_name);
}
